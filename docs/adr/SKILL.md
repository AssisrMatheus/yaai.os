---
name: yaai-adr
description: Architecture Decision Records (ADR NNNN). Use when making a hard-to-reverse decision, checking why the repository is shaped a certain way, or superseding an existing decision.
tags: [adr, index]
---

# Architecture Decision Records

Write an ADR only when all three hold: the choice is **hard to reverse**, **surprising without context**, and a **real trade-off** with genuine alternatives. Copy [[adr/0000-template]] to `docs/adr/NNNN-<slug>.md` using the next 4-digit zero-padded number, and cite it as `ADR NNNN` ([[repo/doc-links]]).

Statuses: `proposed` (under discussion), `accepted` (in force), `superseded` (replaced by a newer ADR; keep the file in place linking its replacement so historical citations stay resolvable).

## Which file to read

| You want to | Read |
| --- | --- |
| Copy the short ADR template for a new decision | [[adr/0000-template]] |
| Understand why docs and ADRs drive code (0001) | [[adr/0001-docs-first]] |
| Understand why Rust is the default language (0002) | [[adr/0002-rust-first]] |
| Understand why the repo targets Unix only (0003) | [[adr/0003-unix-only]] |
| Understand why docs/ is an Obsidian vault indexed by SKILL.md (0004) | [[adr/0004-obsidian-vault-and-skill-indexes]] |
| Understand why superseded docs move to .old/ (0005) | [[adr/0005-superseded-docs-in-dot-old]] |
| Understand why screens are documented before code (0006) | [[adr/0006-screens-as-documents-first]] |
| Understand why .agents/ is the single source for all runtimes (0007) | [[adr/0007-single-agents-source]] |
| Understand why rtk and graphify are the token tools (0008) | [[adr/0008-rtk-and-graphify-token-tools]] |
| Understand the CLI contract for tooling (0009) | [[adr/0009-tooling-cli-contract]] |
| Understand why psychoscript is the shared code mode (0010) | [[adr/0010-psychoscript-code-mode]] |
| Understand why the monorepo uses pnpm workspaces and Turborepo (0011) | [[adr/0011-pnpm-workspaces-with-turborepo]] |
| Understand why Conventional Commits are enforced by husky (0012) | [[adr/0012-conventional-commits-with-husky]] |
| Understand why code-doc links are mechanically checked (0013) | [[adr/0013-checked-doc-code-links]] |
