---
description: Decide to mechanically verify bidirectional links between code, docs, invariants, and ADRs with bin/doc-links.
tags: [adr, docs, tooling]
status: accepted
---

# Checked links between code and docs

On 2026-11-26 the owner decided to verify links between code and documentation mechanically with `bin/doc-links` (`INV-DOCS-007`, [[repo/doc-links]]). Code cites its governing spec, screen, invariant, or ADR in the comment directly above the item, and each governing doc lists covered repository paths under `## Covers`; `bin/doc-links` checks both directions in edit, stop, and commit hooks and feeds edges into [[repo/graphify]].

## Considered options

- **Unchecked prose links between code and docs**: rejected because file renames, heading edits, and code changes silently break traceability and turn documentation into stale commentary.

## Consequences

- Changing covered code requires keeping its governing doc in sync, and broken citations fail fast with exit code `2`.
