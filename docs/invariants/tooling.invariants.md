---
description: CLI contract, exit code, output, and performance invariants for repository tooling.
tags: [invariants, tooling]
---

# Tooling invariants

## INV-TOOLING-001: Every tool runs as a CLI with agent-first, rtk-compatible output and correct exit codes

Active · Decided: 2026-11-26, owner

Every tool under `tooling/` runs non-interactively as a CLI with compact, deterministic, `rtk`-compatible stdout and actionable diagnostics on stderr. Exit codes follow one strict contract: `0` means the operation succeeded and the contract holds; `2` means the calling agent must act and stderr states what to do; `1` means the tool itself or its dependency failed.

Explained in [[best-practices/tooling-cli.rules]], [[repo/rtk]], and [[adr/0009-tooling-cli-contract.adr]].

## INV-TOOLING-002: Tools are fast: cache and work incrementally

Active · Decided: 2026-11-26, owner

Repository tools must execute fast enough to run inside edit and stop hooks without stalling the loop. Process only changed inputs when possible, reuse content-keyed persistent caches with atomic writes, and wire deterministic task work into Turborepo caching.

Explained in [[best-practices/tooling-cli.rules]].

## INV-TOOLING-003: Prefer a reusable tool under tooling/ over a one-off shell or inline-interpreter call

Active · Decided: 2026-11-26, owner

Prefer a reusable tool under `tooling/` over a one-off `python`, `bash` or `bun -e` call whenever the task will run more than once or its flags, paths or parsing would need rediscovering. A one-off inspection (`ls`, `grep`, reading one file) stays a direct call; when in doubt, write the tool.

Explained in [[tooling/SKILL]] and [[best-practices/tooling-cli.rules]].

## INV-TOOLING-004: Check the tooling index before writing a command

Active · Decided: 2026-11-26, owner

Check [[tooling/SKILL]] before writing a repeatable command or a new tool, and reuse an existing tool when one already solves the task.

Explained in [[tooling/SKILL]].

## INV-TOOLING-005: One workspace per tool family; extend before duplicating

Active · Decided: 2026-11-26, owner

Keep one workspace under `tooling/<name>/` per tool family (package `@yaai/<name>`). Add a subcommand to an existing tool before creating a near-duplicate.

Explained in [[tooling/SKILL]] and [[repo/structure]].

## INV-TOOLING-006: Every tool under tooling/ and every entry point in bin/ is registered

Active · Decided: 2026-11-26, owner

Every workspace under `tooling/<name>/` and every executable entry point in `bin/<name>` has a row in [[tooling/SKILL]] and a `docs/tooling/<name>.tool.md` page whose `## Covers` lists the tool's path, added or updated in the same commit that adds or changes the tool. A tool missing from the index does not exist for the next agent. Enforced by `bin/doc-links check`.

Explained in [[tooling/SKILL]], [[repo/doc-kinds#tool]], and [[repo/doc-links]].

## INV-TOOLING-007: Every workspace under tooling/ has a human README

Active · Decided: 2026-11-26, owner

Every workspace under `tooling/<name>/` has a non-empty `README.md` written for humans. Enforced by `bin/doc-links check`.

Explained in [[tooling/SKILL]], [[repo/writing#README files]], and [[repo/doc-links]].
