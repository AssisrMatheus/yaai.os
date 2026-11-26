---
description: Agent runtime layout, audience separation, index routing, and code-mode invariants.
tags: [invariants, agents]
---

# Agent invariants

## INV-AGENTS-001: .agents/ is the single source of skills, hooks and agent roles; runtimes link to it

Active · Decided: 2026-11-26, owner

All project skills, hooks, and agent role definitions live under `.agents/`. Runtime-specific directories (`.claude/`, `.codex/`, `.omp/`, `.pi/`, and similar) contain only symlinks or minimal adapter configs pointing to `.agents/`, never duplicated definitions.

Explained in [[repo/agent-runtimes]], [[repo/hooks]], and [[adr/0007-single-agents-source.adr]].

## INV-AGENTS-002: README files are for humans; AGENTS.md, skills and docs are for agents

Active · Decided: 2026-11-26, owner

Every `README.md` is written short and clear for human readers. `AGENTS.md`, `.agents/skills/`, and `docs/` are written for agent consumption following context-pointer, progressive-disclosure, and single-source-of-truth rules.

Explained in [[repo/writing]].

## INV-AGENTS-003: Agents open docs and skills by task, through indexes, never by reading the folder

Active · Decided: 2026-11-26, owner

Agents discover documentation and skills strictly on demand for the current task by following pointers from `AGENTS.md` through [[SKILL]] and scope `SKILL.md` indexes. Never bulk-read the `docs/` tree into context.

Explained in [[repo/workflow]] and [[SKILL]].

## INV-AGENTS-004: Agents batch tool calls through psychoscript, the repository's code mode

Active · Decided: 2026-11-26, owner

When batching multi-step tool calls or programmatic queries inside an agent turn, use `psychoscript`, the repository's shared code mode, rather than ad-hoc runtime-specific code modes or long sequential tool-call chains.

Explained in [[repo/psychoscript]] and [[adr/0010-psychoscript-code-mode.adr]].
