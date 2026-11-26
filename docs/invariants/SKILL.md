---
name: yaai-invariants
description: Non-negotiable product and repository invariants (INV-<DOMAIN>-NNN). Use when starting any task, proposing a design change, or checking whether a requirement conflicts with an invariant.
tags: [invariants, index]
---

# Invariants

An invariant is a permanent rule that holds across every change in this repository. **Never break, edit, or work around an invariant.** Only the user changes an invariant; if a task conflicts with or is ambiguous against one, stop and tell the user which `INV-<DOMAIN>-NNN` applies and why.

Each invariant carries an immutable ID of the form `INV-<DOMAIN>-NNN` cited by code and docs ([[repo/doc-links]]).

## Which file to read

| You want to | Read |
| --- | --- |
| Check product boundaries (no ads, no unrelated suggestions) | [[invariants/product]] |
| Check docs-first, frontmatter, indexes, history, wikilinks, screens, and glossary rules | [[invariants/docs]] |
| Check supported operating systems and shell assumptions | [[invariants/platform]] |
| Check workspace kinds, import boundaries, and Rust-first language rules | [[invariants/structure]] |
| Check CLI output, exit codes, and tooling performance rules | [[invariants/tooling]] |
| Check structured logging rules across apps, packages, and tooling | [[invariants/logging]] |
| Check `.agents/` single source, audience split, index routing, and code mode rules | [[invariants/agents]] |
