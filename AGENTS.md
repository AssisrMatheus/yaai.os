# AGENTS.md

yaai.os ("yet another ai os") is a home OS: the homepage of a media center, with no ads and no unrelated suggested
content. Rust first; TypeScript with oxfmt and oxlint otherwise. Development is Unix only (macOS, Linux, WSL);
the app itself will support native Windows one day, so shipped code never assumes Unix.

**Mandatory for every line of code:**

1. Follow the best practices from the first line: open [`docs/best-practices/SKILL.md`](docs/best-practices/SKILL.md)
   and the rule page for what you are writing before you write it.
2. Before reporting a code change done, run the `yaai-code-review` skill on your own diff. Its findings are blocking:
   fix or justify each one.

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
- **Repository tools** live in [`docs/tooling/SKILL.md`](docs/tooling/SKILL.md): check it before writing a one-off command.

Details for each: [`docs/repo/SKILL.md`](docs/repo/SKILL.md).

## Skills

| Skill | When |
| --- | --- |
| `yaai-code-review` | **Mandatory** before reporting done any change to code, shell, hooks or tool config |
| `yaai-data-review` | Schema, migration, seed, query or transaction work, once a data store exists |
| `yaai-workflow` | Any task that changes behaviour, rules, screens or code |
| `yaai-doc-change` | Creating or changing anything under `docs/` |
| `yaai-screen` | Adding, changing or implementing a screen or subview |
| `yaai-tooling` | Before a one-off script, or adding or changing a tool or `bin/` entry point |
| `yaai-delegate` | Splitting work across sub-agents or asking for black-box verification |
| `graphify` | Questions about the codebase, its structure or relationships |
| `turborepo` | `turbo.json`, task pipelines, caching, filtering |

## Git

Conventional Commits, split by scope, made often. Never `--no-verify`. Sub-agents do not commit.
