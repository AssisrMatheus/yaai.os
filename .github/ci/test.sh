#!/usr/bin/env bash
# spec: docs/repo/ci.md#affected-tests
# BP-CI-023, BP-CI-024
set -euo pipefail
checkout=$(pwd -P)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir "$work/suites"
ln -s "$checkout" "$work/checkout"
filter=()
hooks=true
if [[ -n ${TURBO_SCM_BASE:-} ]]; then
  filter+=(--affected)
  git diff --no-renames --name-only -z "$TURBO_SCM_BASE...${TURBO_SCM_HEAD:?}" > "$work/paths"
  hooks=false
  while IFS= read -r -d '' path; do
    case "$path" in
      docs/*|*.md) ;;
      .agents/hooks/*|.agents/hooks.json|.github/*|.claude/settings.json|.codex/hooks.json|bin/agent-pick|bin/test-shard) hooks=true ;;
      */*) ;;
      *) hooks=true ;;
    esac
  done < "$work/paths"
fi
pnpm exec turbo run test --continue "${filter[@]}" --dry=json > "$work/graph.json"
jq -r '.tasks[] | select(.task == "test" and .command != "<NONEXISTENT>") | [.directory, .command] | @tsv' "$work/graph.json" > "$work/tasks"
: > "$work/dirs"
while IFS=$'\t' read -r dir command; do
  if [[ $command =~ ^cargo\ test\ --locked\ -p\ ([a-zA-Z0-9_-]+)$ ]]; then
    crate=${BASH_REMATCH[1]}
    if [[ ! -e $work/cargo.json ]]; then
      cargo metadata --locked --no-deps --format-version 1 > "$work/cargo.json"
    fi
    # Nonstandard targets and feature gates retain the workspace command's semantics.
    jq -er --arg crate "$crate" --arg dir "$dir" '
      .packages[] | select(.name == $crate) | .targets |
      if all(.[]; (."required-features" == null) and
        (.kind[0] | IN("lib", "bin", "test", "custom-build"))) then
        [.[] | (select(.test) | [$dir, .kind[0], .name] | join("/")),
          (select(.doctest) | [$dir, "doc", .name] | join("/"))] |
        if length > 0 then .[] else $dir end
      else $dir end' "$work/cargo.json" >> "$work/dirs"
  elif [[ $command == 'bun test' && ! -e $dir/bunfig.toml ]]; then
    git -c core.quotepath=false ls-files -co --exclude-standard -- "$dir" > "$work/paths"
    # Git quotes unusual names. Delegate discovery to Bun when a line cannot represent a path.
    if grep -q '^"' "$work/paths"; then
      echo "$dir" >> "$work/dirs"
    else
      awk '!/\/\./ && /[._](test|spec)\.([jt]sx?|[cm][jt]s)$/ {print}' "$work/paths" > "$work/files"
      if [[ -s $work/files ]]; then cat "$work/files"; else echo "$dir"; fi >> "$work/dirs"
    fi
  else
    echo "$dir" >> "$work/dirs"
  fi
done < "$work/tasks"
if "$hooks"; then echo root-hooks >> "$work/dirs"; fi
out=${CI_TIMINGS_OUT:-$work/observed.json}
if [[ -s $work/dirs ]]; then
  seed=${CI_TIMINGS_BASE:-$checkout/.github/ci/timings.json}
  sed 's/$/.test.ts/' "$work/dirs" |
    bin/test-shard --timings "$seed" --shard "$1" --count "$2" --json > "$work/selected.json"
  jq -r '.files[]' "$work/selected.json" > "$work/selected"
else
  : > "$work/selected"
fi
if [[ ! -s $work/selected ]]; then
  echo '{"version":1,"files":{}}' > "$out"
  echo "No test units for shard $1/$2"
  exit 0
fi
while IFS= read -r file; do
  mkdir -p "$work/suites/$(dirname "$file")"
  cp .github/ci/suite.ts "$work/suites/$file"
done < "$work/selected"
cd "$work/suites"
bun test --timings="$work/measured.json" --update-timings
cp "$work/measured.json" "$out"
