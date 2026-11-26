---
description: Documentation encyclopedia, history, link, screen, and glossary invariants.
tags: [invariants, docs]
---

# Documentation invariants

## INV-DOCS-001: docs/ is the source of truth; code reproduces it

Active · Decided: 2026-11-26, owner

Behaviour, rules, decisions, and screens are defined under `docs/` before code is written. When code and a governing doc disagree, the code is wrong; a doc changes only when the user driving the agent decides to change it.

Explained in [[repo/workflow]] and [[adr/0001-docs-first.adr]].

## INV-DOCS-002: Every file under docs/ has the short frontmatter

Active · Decided: 2026-11-26, owner

Every Markdown file under `docs/` opens with YAML frontmatter containing `description` (one sentence) and `tags` (a YAML list). Scope `SKILL.md` indexes add `name` first, and ADRs add `status`; keep frontmatter free of extra keys unless a repository tool requires them.

Explained in [[repo/documentation]].

## INV-DOCS-003: Every scope has a SKILL.md index; every doc is reachable from docs/SKILL.md

Active · Decided: 2026-11-26, owner

Every scope directory under `docs/` contains a `SKILL.md` index linking every active page in that scope, and [[SKILL]] links every scope's `SKILL.md`. No active documentation file may be orphaned from the root index.

Explained in [[repo/documentation]] and [[SKILL]].

## INV-DOCS-004: A superseded doc moves to its scope's .old/ with the replacement and the commit

Active · Decided: 2026-11-26, owner

When active documentation is replaced, move the superseded document or section to that scope's `.old/` folder and record both the replacement doc link and the commit that superseded it. Keep `.old/` excluded from `graphify` so archived text never pollutes active queries.

Explained in [[repo/history]] and [[adr/0005-superseded-docs-in-dot-old.adr]].

## INV-DOCS-005: Docs link each other with Obsidian wikilinks; docs/ opens as an Obsidian vault

Active · Decided: 2026-11-26, owner

Every link between files under `docs/` uses a vault-root Obsidian wikilink without `.md` (for example `[[repo/hooks]]`), never a relative Markdown path. Keep `docs/.obsidian/app.json` configured for vault-root wikilinks so `docs/` opens cleanly as an Obsidian vault.

Explained in [[repo/documentation]] and [[adr/0004-obsidian-vault-and-skill-indexes.adr]].

## INV-DOCS-006: A doc written from owner-given references lists them in References

Active · Decided: 2026-11-26, owner

Whenever the owner supplies reference URLs for a topic, the resulting page ends with a `## References` section listing those URLs so future sessions can fetch up-to-date material. Prefer an `llms.txt` or `.md` endpoint over HTML when one exists, and use only that URL.

Explained in [[repo/documentation]].

## INV-DOCS-007: Code cites the doc that governs it, and that doc lists the code in Covers

Active · Decided: 2026-11-26, owner

Every governed code item cites its governing rule in the comment directly above it (`spec: docs/<path>.md#<slug>`, `screen: docs/screens/<name>.screen.md`, `INV-<DOMAIN>-NNN`, or `ADR NNNN`), and the governing doc lists the governed repository-relative paths in a `## Covers` section. `bin/doc-links check` verifies both directions.

Explained in [[repo/doc-links]] and [[adr/0013-checked-doc-code-links.adr]].

## INV-DOCS-008: Every screen and subview has an implementation-neutral doc under docs/screens/; its code cites it

Active · Decided: 2026-11-26, owner

Every user-facing screen and subview is specified first in an implementation-neutral document under `docs/screens/` describing states, flows, and content without framework details. The implementing UI code cites that screen doc via `screen: docs/screens/<name>.screen.md`.

Explained in [[screens/SKILL]] and [[adr/0006-screens-as-documents-first.adr]].

## INV-DOCS-009: Names in code and docs use the glossary's terms

Active · Decided: 2026-11-26, owner

Domain concepts use the exact canonical terms defined in [[glossary]] across code identifiers, types, UI copy, and documentation. Avoid synonyms listed under `_Avoid_`, and add new domain terms to [[glossary]] before using them in code.

Explained in [[glossary]].
