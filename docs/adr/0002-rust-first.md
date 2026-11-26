---
description: Decide on Rust as the default language for apps, packages, and tooling, with TypeScript as the secondary choice.
tags: [adr, structure, rust]
status: accepted
---

# Rust first

On 2026-11-26 the owner decided to write new apps, packages, and repository tooling in Rust by default, falling back to TypeScript (formatted with `oxfmt`, linted with `oxlint`, run with `bun`, managed with `pnpm`) only where Rust is not the right fit (`INV-STRUCTURE-002`, [[best-practices/rust]], [[best-practices/typescript]]). Rust gives fast startup, strong compile-time guarantees, and single-binary CLI tooling without runtime overhead.

## Considered options

- **TypeScript everywhere**: rejected because repository tooling and core OS components benefit from Rust's native speed, memory safety, and zero-runtime binaries.

## Consequences

- Best-practice guides are written Rust-first, and other language guides reference [[best-practices/rust]] rather than duplicating shared rules.
