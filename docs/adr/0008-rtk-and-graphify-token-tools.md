---
description: Decide to adopt rtk for compact command output and graphify for AST code-and-doc graph queries.
tags: [adr, tooling, agents]
status: accepted
---

# rtk and graphify as the agents' token tools

On 2026-11-26 the owner decided to wire `rtk` for condensed shell output and `graphify` (AST mode) for local code-and-documentation graph queries ([[repo/rtk]], [[repo/graphify]]). Condensing verbose command output and querying graph relationships before raw file reads cut token waste while keeping every diagnostic signal recoverable.

## Considered options

- **Raw shell output and raw grep/read exploration**: rejected because verbose logs and broad file scans exhaust context windows quickly and degrade attention on the actual task.

## Consequences

- Both tools are wired inside the repository by hand (never via global `rtk init` or `graphify <agent> install`), and Git hooks keep `graphify-out/` refreshed after commits.
