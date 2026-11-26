---
description: Decide to store all skills, hooks, and agent roles once under .agents/ and link every runtime directory to it.
tags: [adr, agents]
status: accepted
---

# One .agents/ source for every agent runtime

On 2026-11-26 the owner decided to store all repository skills, hooks, and agent role files once under `.agents/` and wire runtime directories (`.claude/`, `.codex/`, `.omp/`, `.pi/`) through symlinks or thin adapter configs (`INV-AGENTS-001`, [[repo/agent-runtimes]]). A single source prevents instructions and enforcement hooks from drifting across agent runtimes.

## Considered options

- **Per-runtime copies in `.claude/`, `.codex/`, `.omp/`, and `.pi/`**: rejected because every edit would have to be repeated across runtimes and divergence would break uniform behaviour.

## Consequences

- New skills and hooks land under `.agents/` first; runtime folders hold only symlinks or minimal wiring.
