---
description: Decide to enforce Conventional Commits and staged checks via husky, commitlint, and lint-staged.
tags: [adr, repo, commits]
status: accepted
---

# Conventional Commits enforced by husky

On 2026-11-26 the owner decided to enforce Conventional Commits (`commitlint` via `.husky/commit-msg`) alongside staged formatting/linting (`lint-staged` via `.husky/pre-commit`) and `bin/doc-links commit` ([[repo/commits]], [[repo/doc-links]]). Enforcing commit structure and link checks in Git hooks keeps history machine-readable and blocks broken citations before they land.

## Considered options

- **Free-form commit messages checked only in review or CI**: rejected because inconsistent messages break automated changelog/history queries and catch errors too late in the loop.

## Consequences

- Agent hooks block `git --no-verify` so commit checks cannot be bypassed silently.
