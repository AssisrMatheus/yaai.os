#!/usr/bin/env bash
# spec: docs/repo/ci.md#timing-and-caching
set -euo pipefail
# Both shards choose the same successful main push as of this run's creation.
run=$(gh api "repos/$GITHUB_REPOSITORY/actions/runs/$GITHUB_RUN_ID")
cutoff=$(jq -r .created_at <<< "$run")
workflow=$(jq -r .workflow_id <<< "$run")
runs=$(gh api --method GET "repos/$GITHUB_REPOSITORY/actions/workflows/$workflow/runs" \
  -f branch=main -f event=push -f status=success -f "created=<$cutoff" -F per_page=100)
id=$(jq -r --arg cutoff "$cutoff" '[.workflow_runs[] | select(.updated_at < $cutoff)][0].id // empty' <<< "$runs")
cp .github/ci/timings.json "$RUNNER_TEMP/base.json"
if [[ -n $id ]]; then
  artifact=$(gh api "repos/$GITHUB_REPOSITORY/actions/runs/$id/artifacts?name=main-timings" \
    --jq '.artifacts[] | select(.expired == false) | .id')
  if [[ -n $artifact ]]; then
    gh api "repos/$GITHUB_REPOSITORY/actions/artifacts/$artifact/zip" > "$RUNNER_TEMP/timings.zip"
    unzip -p "$RUNNER_TEMP/timings.zip" base.json > "$RUNNER_TEMP/base.json"
  fi
fi
