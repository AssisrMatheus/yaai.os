---
name: yaai-adr
description: Architecture Decision Records (ADR NNNN). Use when making a hard-to-reverse decision, checking why the repository is shaped a certain way, or superseding an existing decision.
tags: [adr, index]
---

# Architecture Decision Records

Write an ADR only when all three hold: the choice is **hard to reverse**, **surprising without context**, and a **real trade-off** with genuine alternatives. Copy [[adr/0000-template.adr]] to `docs/adr/NNNN-<slug>.adr.md` using the next 4-digit zero-padded number, and cite it as `ADR NNNN` ([[repo/doc-links]]). The `.adr.md` ending makes it an ADR: `status` in the frontmatter, one H1, an opening paragraph, and only the optional `Considered options` and `Consequences` sections ([[repo/doc-kinds#adr]]).

Statuses: `proposed` (under discussion), `accepted` (in force), `superseded` (replaced by a newer ADR; keep the file in place linking its replacement so historical citations stay resolvable).

## Which file to read

| You want to | Read |
| --- | --- |
| Copy the short ADR template for a new decision | [[adr/0000-template.adr]] |
| Understand why docs and ADRs drive code (0001) | [[adr/0001-docs-first.adr]] |
| Understand why Rust is the default language (0002) | [[adr/0002-rust-first.adr]] |
| Understand why the repo targets Unix only (0003) | [[adr/0003-unix-only.adr]] |
| Understand why docs/ is an Obsidian vault indexed by SKILL.md (0004) | [[adr/0004-obsidian-vault-and-skill-indexes.adr]] |
| Understand why superseded docs move to .old/ (0005) | [[adr/0005-superseded-docs-in-dot-old.adr]] |
| Understand why screens are documented before code (0006) | [[adr/0006-screens-as-documents-first.adr]] |
| Understand why .agents/ is the single source for all runtimes (0007) | [[adr/0007-single-agents-source.adr]] |
| Understand why rtk and graphify are the token tools (0008) | [[adr/0008-rtk-and-graphify-token-tools.adr]] |
| Understand the CLI contract for tooling (0009) | [[adr/0009-tooling-cli-contract.adr]] |
| Understand why psychoscript is the shared code mode (0010) | [[adr/0010-psychoscript-code-mode.adr]] |
| Understand why the monorepo uses pnpm workspaces and Turborepo (0011) | [[adr/0011-pnpm-workspaces-with-turborepo.adr]] |
| Understand why Conventional Commits are enforced by husky (0012) | [[adr/0012-conventional-commits-with-husky.adr]] |
| Understand why code-doc links are mechanically checked (0013) | [[adr/0013-checked-doc-code-links.adr]] |
