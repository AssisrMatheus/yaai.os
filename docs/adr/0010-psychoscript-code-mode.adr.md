---
description: Decide to use psychoscript as the repository's shared code mode for batching agent tool calls.
tags: [adr, agents, tooling]
status: accepted
---

# psychoscript as code mode

On 2026-11-26 the owner decided to standardize on `psychoscript` as the repository's code mode for batching multi-step agent tool calls (`INV-AGENTS-004`, [[repo/psychoscript]]). One shared code mode gives every agent runtime a consistent way to compose tool calls and cut multi-turn round trips without tying workflows to a single harness.

## Considered options

- **Each runtime's native code mode**: rejected because every runtime exposes a different syntax, sandbox, and capability surface, fragmenting skills and prompts across harnesses.

## Consequences

- Skills and briefs reference `psychoscript` rather than runtime-specific code-mode instructions.
