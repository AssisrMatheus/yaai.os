# AGENTS.md

yaai.os ("yet another ai os") is a home OS: the homepage of a media center, with no ads and no unrelated suggested
content. Rust first; TypeScript with oxfmt and oxlint otherwise. Unix only (macOS, Linux, WSL).

## Docs come first

`docs/` is the source of truth and code is its reproduction. When code and a doc disagree, the code is wrong. A doc
changes only when the user decides it.

- Start at [`docs/SKILL.md`](docs/SKILL.md). It routes to one scope index, which routes to one page. Open pages by
  task; never read the folder.
- Never break, edit or work around an invariant ([`docs/invariants/SKILL.md`](docs/invariants/SKILL.md)). If a task
  conflicts with one, stop and say which and why.
- Names come from [`docs/glossary.md`](docs/glossary.md).
- Before changing behaviour, a rule or a screen, load the `yaai-workflow` skill.

## Commands

```sh
pnpm install            # also installs the git hooks
pnpm lint               # workspaces, root files, then docs and code links
pnpm test
pnpm format
bin/doc-links check     # links between code and docs, and the shape of docs/
```

Packages are managed with pnpm and TypeScript runs with bun. Hooks format edited files and run the checks when you
stop; a hook that exits 2 tells you what to do next.

## Tools you already have

- **rtk** condenses command output. Treat it as complete; `rtk recall <hash>` restores a truncated result.
- **graphify** answers "where is X" and "what calls Y". Ask it before grepping: `graphify query "<question>"`.
- **psychoscript** is the code mode: batch independent tool calls in one program.

Details for each: [`docs/repo/SKILL.md`](docs/repo/SKILL.md).

## Git

Conventional Commits, split by scope, made often. Never `--no-verify`. Sub-agents do not commit.
