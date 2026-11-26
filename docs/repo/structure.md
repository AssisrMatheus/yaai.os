---
description: The workspace kinds of the monorepo, what each owns, how to add a workspace, and which language to use.
tags: [repo, structure]
---

# Repository structure

The repository is a pnpm workspace orchestrated by Turborepo ([[repo/turborepo/tasks]]) with a Cargo workspace for
Rust. Each top-level directory has one role (INV-STRUCTURE-001, ADR 0011).

| Path         | Owns                                                                         | Rules                                                                                                                                                             |
| ------------ | ---------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apps/*`     | Deployable, shipped apps                                                     | An app never imports another app. Shared code moves to `packages/*`. Empty today (`.gitkeep`).                                                                    |
| `packages/*` | Reusable code used by apps, tooling or other packages                        | Shared lint configuration and Rust checks are documented in [[repo/lint-rules]].                                                                                  |
| `tooling/*`  | CLIs and automation for the process                                          | Never shipped. Rust first. Each tool runs as a CLI ([[best-practices/tooling-cli.rules]]).                                                                        |
| `bin/`       | Small committed POSIX `sh` entry points that wrap tooling (`bin/agent-pick`) | No logic beyond locating and running the tool.                                                                                                                    |
| `vendor/`    | External sources kept for reading                                            | Git-ignored (only `vendor/.gitkeep` is tracked), read-only: never edited, linted, formatted or committed. The shell and edit hooks block writes ([[repo/hooks]]). |
| `docs/`      | The encyclopedia ([[repo/documentation]])                                    | Source of truth.                                                                                                                                                  |
| `.agents/`   | Skills, hooks and agent roles for every runtime ([[repo/agent-runtimes]])    | INV-AGENTS-001.                                                                                                                                                   |

`pnpm-workspace.yaml` registers `apps/*`, `packages/*` and `tooling/*`. `Cargo.toml` lists the Rust members.

## Languages

Rust first (INV-STRUCTURE-002, ADR 0002; [[best-practices/rust.rules]]). Use TypeScript only when Rust does not fit:
formatted by `oxfmt`, linted by `oxlint`, run with `bun`, packages installed by `pnpm`. Other languages follow
[[best-practices/typescript.rules]], which refers to the Rust page. The Rust toolchain is pinned in `rust-toolchain.toml`.

## Package names

A workspace is named `@yaai/<directory>`: `tooling/doc-links` is `@yaai/doc-links`. Every workspace is
`"private": true`.

## Where dependencies go

Install a dependency in the workspace whose code imports it: `pnpm add <pkg> --filter @yaai/<dir>`. Root
`devDependencies` hold repository tooling only (turbo, husky, lint-staged, commitlint, oxfmt, oxlint). Rust
dependencies go in the member crate's `Cargo.toml`.

## Add a workspace

Done when `pnpm install` succeeds and `pnpm lint` exits 0.

1. Create `apps/<name>`, `packages/<name>` or `tooling/<name>`. Pick the kind from the table above.
2. Write its own `README.md` for humans, under 40 lines ([[repo/writing]]).
3. Add `package.json` named `@yaai/<name>` with the scripts it supports (`build`, `lint`, `test`, `typecheck`,
   `dev`) so Turborepo runs it. A Rust tool wraps Cargo in these scripts ([[repo/turborepo/tasks]]).
4. A Rust crate also joins `members` in the root `Cargo.toml` and inherits the workspace lints.
5. Cite the governing doc in the code and list the code in that doc's `## Covers` ([[repo/doc-links]]).
6. A tool that agents call gets a `bin/<name>` wrapper (`#!/bin/sh`, executable).
7. Run `pnpm install`, then `pnpm lint`.

## Covers

- `pnpm-workspace.yaml`
- `Cargo.toml`
- `rust-toolchain.toml`
- `.oxfmtrc.json`
- `.oxlintrc.json`
- `.gitignore`
