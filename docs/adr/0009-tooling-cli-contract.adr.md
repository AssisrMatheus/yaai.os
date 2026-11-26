---
description: Decide on a uniform agent-first CLI contract for every repository tool under tooling/.
tags: [adr, tooling]
status: accepted
---

# The CLI contract for tooling

On 2026-11-26 the owner decided that every tool under `tooling/` exposes a fast, non-interactive CLI with compact, `rtk`-compatible stdout, actionable stderr, and strict exit codes (`0` ok, `2` calling agent must act, `1` internal tool failure) (`INV-TOOLING-001`, `INV-TOOLING-002`, [[best-practices/tooling-cli.rules]]). Uniform exit codes and compact output let agent hooks and sub-agents distinguish actionable feedback from tool crashes without parsing prose.

## Considered options

- **Ad-hoc scripts with human-formatted output and generic exit code `1`**: rejected because hooks cannot separate fixable contract violations from internal failures and noisy output wastes tokens.

## Consequences

- Every tool supports `--help`, `--json` where result-producing, incremental/cached execution, and thin POSIX wrappers in `bin/`.
