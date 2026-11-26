---
description: The Turborepo tasks of this monorepo, why checks depend on transit, how root scripts delegate, and how a Rust tool joins the task graph.
tags: [repo, turborepo, tasks, cache]
---

# Turborepo tasks

Turborepo runs a workspace's `package.json` script of the same name and caches the result by input hash (ADR 0011,
INV-TOOLING-002). General Turborepo guidance is in [[repo/turborepo/SKILL]].

## Tasks in `turbo.json`

| Task        | Depends on | Cache                  | Notes                                                    |
| ----------- | ---------- | ---------------------- | -------------------------------------------------------- |
| `build`     | `^build`   | yes, outputs `dist/**` | Dependencies build first.                                |
| `dev`       | none       | no, persistent         | Long-running.                                            |
| `lint`      | `transit`  | yes, no outputs        |                                                          |
| `lint:fix`  | `transit`  | no                     | Writes files.                                            |
| `test`      | `transit`  | yes, no outputs        |                                                          |
| `typecheck` | `transit`  | yes, no outputs        |                                                          |
| `transit`   | `^transit` | n/a                    | Declares the dependency graph for checks; has no script. |

`globalPassThroughEnv` hands `CARGO_HOME`, `RUSTUP_HOME` and `RUSTUP_TOOLCHAIN` to tasks without hashing them.

## Why `transit`

`lint`, `test` and `typecheck` depend on `transit`, not on `^lint`. `^lint` would run every dependency's lint before
this package's lint, so a failure or an edit in a leaf package serialises the graph. `transit` links the same
packages without running anything, so a package's check reruns when a dependency's source changes (the hash includes
it) while all checks still run in parallel.

## Root scripts

Root `package.json` scripts only delegate: `build`, `dev`, `lint:fix`, `test`, `typecheck` run `turbo run <task>`.
Checks that belong to no workspace are appended, never reimplemented:

| Script                   | Runs                                                                                            |
| ------------------------ | ----------------------------------------------------------------------------------------------- |
| `lint`                   | `turbo run lint --filter=@yaai/lint` for annotated workspace diagnostics, `bin/doc-links check` |
| `lint:fix`               | `turbo run lint:fix`, `oxlint --fix` on root files                                              |
| `test`                   | `turbo run test`, `bun test .agents/hooks`                                                      |
| `format`, `format:check` | `oxfmt` over the repository                                                                     |

## A Rust tool joins Turborepo

Turborepo sees only `package.json`. A tool under `tooling/<name>` has one that wraps Cargo, as
`tooling/doc-links/package.json` does:

```json
{
  "scripts": {
    "build": "cargo build --release --locked -p <crate>",
    "test": "cargo test --locked -p <crate>",
    "lint": "cargo fmt --check -p <crate> && cargo clippy --locked -p <crate> --all-targets -- -D warnings"
  }
}
```

Add the crate to `members` in `Cargo.toml` ([[repo/structure#Add a workspace]]).

The lint workspace runs the repository check once, including the live Cargo advisory database. Its task bypasses the Turbo cache so advisory updates and Git-index changes are visible. Other workspace lint scripts accept their own scope.

## Caching

Pure, deterministic tasks are cached: give each `outputs` that match what it writes (`dist/**` for build, none for
checks). A task that writes source or runs forever sets `"cache": false`. A tool must also cache inside its own
process by content hash so the hooks stay fast (INV-TOOLING-002).

## Covers

- `turbo.json`
- `package.json`
- `tooling/doc-links/package.json`

## References

- [Turborepo docs index](https://turborepo.com/llms.txt)
- [Turborepo skill](https://github.com/vercel/turborepo/blob/main/skills/turborepo/SKILL.md)
