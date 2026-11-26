---
name: yaai-tooling
description: Procedure for reusing, extending, or adding a repository tool under tooling/ or an entry point under bin/. Use when about to write a one-off script or long shell command, or when adding or changing a tool or bin/ entry point.
---

# Repository tooling

Facts behind each step: `docs/tooling/SKILL.md` (tool index), `docs/invariants/tooling.invariants.md` (`INV-TOOLING-001`..`007`), `docs/best-practices/tooling-cli.rules.md` (CLI output and exit codes), `docs/repo/structure.md` (workspace layout), `docs/repo/writing.md` (human `README.md`), `docs/repo/doc-kinds.md#tool` (`.tool.md` shape).

1. **Check the index first.** Open `docs/tooling/SKILL.md`. If an existing tool already solves the task, run it instead of writing a new command (`INV-TOOLING-004`). Done when you know whether to reuse, extend, or create.
2. **Decide tool vs direct call.** Keep a one-off inspection (`ls`, `grep`, reading one file) as a direct call. Build or extend a tool whenever the task will run more than once or its flags, paths or parsing would need rediscovering; when in doubt, write the tool (`INV-TOOLING-003`).
3. **Extend before creating.** If a tool family already exists under `tooling/`, add a subcommand to it instead of creating a near-duplicate workspace (`INV-TOOLING-005`).
4. **Create the workspace (when new).**
   - Pick Rust first (`INV-STRUCTURE-002`); use TypeScript on bun only when Rust does not fit.
   - Add `tooling/<name>/package.json` named `@yaai/<name>` (`"private": true`) exposing Turborepo scripts (`build`, `test`, `lint`, `start`), and for Rust add `tooling/<name>` to root `Cargo.toml` members inheriting workspace lints.
   - Follow the CLI output, exit-code (`0`/`2`/`1`), rtk and caching contract in `docs/best-practices/tooling-cli.rules.md` (`INV-TOOLING-001`, `INV-TOOLING-002`).
   - Add a `bin/<name>` POSIX `sh` executable wrapper when agents call the tool directly.
   - Write a short human `tooling/<name>/README.md` under 40 lines (`INV-TOOLING-007`, `docs/repo/writing.md#readme-files`).
5. **Register in the same commit.** Add or update `docs/tooling/<name>.tool.md` (required H2s in order: `Synopsis`, `Commands`, `Options`, `Exit codes`, `Output`, `Covers` listing `tooling/<name>/` and/or `bin/<name>`) and its row in `docs/tooling/SKILL.md` (`INV-TOOLING-006`). Cite governing docs in the tool's source and tests.
6. **Verify.** Run `bin/doc-links check` plus the tool's `lint` and `test` scripts. Done when all exit 0.
