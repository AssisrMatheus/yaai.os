---
description: Decide to archive superseded documentation inside each scope's .old/ folder with replacement and commit metadata.
tags: [adr, docs, history]
status: accepted
---

# Superseded docs are kept in .old/

On 2026-11-26 the owner decided to move superseded documentation into the scope's `.old/` folder, recording the replacement doc link and the superseding commit (`INV-DOCS-004`, [[repo/history]]). Keeping archived docs beside their scope preserves design rationale without cluttering active indexes, Obsidian search, or `graphify` queries.

## Considered options

- **Delete superseded text and rely solely on Git history**: rejected because digging through Git log for discarded specs is slow and easy to miss.
- **Keep deprecated sections inline**: rejected because stale rules pollute agent context and risk being followed by mistake.

## Consequences

- `.old/` is excluded in `.graphifyignore` and hidden by Obsidian by default.
