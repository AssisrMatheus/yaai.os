---
description: Why custom Rust syntax lints run on the workspace stable toolchain.
tags: [adr, rust, lint]
status: accepted
---

# Run custom Rust syntax lints on stable

On 2026-11-26 the lint packages adopted `syn` for file-local Rust checks. A post-edit hook can parse one file
without compiling its crate or installing another compiler (INV-TOOLING-002).

## Considered options

Dylint exposes compiler type information but requires cargo-dylint and a matching nightly compiler and driver.
Keep type-dependent diagnostics in stock Clippy. Syntax checks must stay within what the parsed file proves.
