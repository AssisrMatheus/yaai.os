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
