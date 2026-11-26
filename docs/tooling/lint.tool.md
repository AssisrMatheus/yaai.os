---
description: Run repository linters with rule references and machine-readable findings.
tags: [tooling, lint]
---

# lint

## Synopsis

`bin/lint [--json] [paths...]`

## Commands

Run from the checkout root. With no paths, check the workspace. Pass a workspace directory or individual source files for post-edit checks.

CI invokes this same command. The workspace manifest mirrors the Cargo dependency on the shared Rust lint package for affected filtering ([[repo/ci]]). Full runs require cargo-machete and cargo-deny on PATH. Install the JS dependencies with `pnpm install`.

## Options

`--json` writes one object to stdout with `total` and the ordered `findings` array. Each row contains `path`, `line`, `lint`, `message` and `rules`.

`-v` or `--verbose` emits structured backend events to stderr.

## Exit codes

0 clean, 2 findings requiring edits, 1 tool failure.

## Output

Findings go to stderr in file and line order with the lint, action and governing rule. A summary follows.
See [[repo/lint-rules]] for the binding contract.

## Covers

- `tooling/lint/`
- `bin/lint`
