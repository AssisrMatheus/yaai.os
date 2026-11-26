---
description: Monorepo directory layout, workspace boundary, and language-choice invariants.
tags: [invariants, structure]
---

# Structure invariants

## INV-STRUCTURE-001: Workspace kinds (apps, packages, tooling, vendor, bin); an app never imports an app

Active · Decided: 2026-11-26, owner

Top-level directories have fixed roles: `apps/` for deployable apps, `packages/` for shared libraries, `tooling/` for non-shipped CLIs and automation, `bin/` for small committed POSIX wrappers, and `vendor/` for git-ignored read-only external sources. An app under `apps/` never imports another app; shared code belongs in `packages/`.

Explained in [[repo/structure]].

## INV-STRUCTURE-002: Rust first; otherwise TypeScript with oxfmt and oxlint

Active · Decided: 2026-11-26, owner

New apps, packages, and tooling default to Rust. Where Rust is not the right fit, use TypeScript formatted with `oxfmt`, linted with `oxlint`, executed with `bun`, and managed with `pnpm`; never introduce alternative JS package managers or TypeScript runners.

Explained in [[best-practices/rust]], [[best-practices/typescript]], and [[adr/0002-rust-first.adr]].
