---
description: Decide that docs/ and ADRs are the source of truth and code is their reproduction.
tags: [adr, workflow, docs]
status: accepted
---

# Docs-first: decisions and docs drive code

On 2026-11-26 the owner decided to make `docs/` the authoritative source of truth for behaviour, rules, decisions, and screens, treating code as a faithful reproduction of those documents (`INV-DOCS-001`, [[repo/workflow]]). Updating documentation before code prevents silent drift between intent and implementation and gives agents a compact, reviewable contract before edits begin.

## Considered options

- **Code as truth with docs as commentary**: rejected because docs go stale as soon as code diverges and agents cannot reliably infer unstated rules from implementation details alone.

## Consequences

- When code and a governing doc disagree, the code is fixed unless the user explicitly changes the doc.
