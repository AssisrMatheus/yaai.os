---
description: How commits are written and gated by Conventional Commits, commitlint, lint-staged and the doc-links gate, and who commits.
tags: [repo, commits, git]
---

# Commits

Conventional Commits, enforced by husky (ADR 0012). `pnpm install` activates the hooks through the `prepare` script.

## Format

```text
<type>(<scope>): <subject>

<body: why, when the subject is not enough>

<trailers>
```

Types come from `@commitlint/config-conventional` (`feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `perf`,
`build`, `ci`, `style`, `revert`). The scope names the area (`hooks`, `doc-links`, `docs(specs)`). Breaking changes
use `!` or a `BREAKING CHANGE:` footer. The spec is the
[Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).

## What runs

| Hook | Runs | On |
| --- | --- | --- |
| `.husky/pre-commit` | `lint-staged` | staged files only |
| `.husky/commit-msg` | `commitlint --edit`, then `bin/doc-links commit <msg-file>` when the binary exists | the message and staged docs/code |
| `.husky/post-commit` | graphify rebuild in the background ([[repo/graphify]]) | the repository |

`lint-staged.config.mjs` maps files to commands:

| Staged files | Commands |
| --- | --- |
| `*.{js,cjs,mjs,ts,cts,mts,jsx,tsx}` | `oxlint --deny-warnings --fix`, then `oxfmt` |
| `*.{json,jsonc,yaml,yml,toml}` | `oxfmt` |
| `*.rs` | `rustfmt --edition 2024` |

Markdown is outside `oxfmt` (`.oxfmtrc.json` ignores it); its checks are the doc-links gate.

## The doc-links gate

`bin/doc-links commit` blocks a commit (exit 2) for an unresolved citation, a staged doc that fails the frontmatter,
link or index checks, or covered code staged without its doc. A change that leaves the contract as it is carries a
`Spec-unchanged: <doc> <reason>` trailer. Full rules and the trailer form are in [[repo/doc-links#The commit gate]].

## Practice

- Split commits by scope and context: one logical change per commit, never a mixed dump. Commit often; push at the end.
- Never `--no-verify` or `commit -n`. The shell policy blocks them ([[repo/hooks#Shell policy]]). Fix the reported
  check instead.
- The orchestrator commits. Sub-agents never run `git add`, `commit`, `stash`, `restore` or `reset`
  ([[repo/delegation]]).
- A doc change and the code that reproduces it can share a commit when they are one logical change
  ([[repo/workflow]]).

## Covers

- `.husky/pre-commit`
- `.husky/commit-msg`
- `lint-staged.config.mjs`
- `commitlint.config.mjs`

## References

- [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)
- [commitlint](https://commitlint.js.org/)
- [lint-staged](https://github.com/lint-staged/lint-staged)
- [Husky](https://typicode.github.io/husky/)
