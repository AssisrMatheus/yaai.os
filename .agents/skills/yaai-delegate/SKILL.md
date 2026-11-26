---
name: yaai-delegate
description: Sub-agent dispatch and brief writing for yaai.os. Use when splitting parallel independent work across sub-agents, requesting black-box verification, or isolating a context-heavy lookup.
---

# yaai.os delegation

Follow `docs/repo/delegation.md` for full policy and the model classifier table.

## Dispatch procedure

1. **Apply the main-loop-first gate**: keep chained context-sharing work and small edits in the main session. Delegate only parallel disjoint slices, black-box verification, or context-heavy lookups. Split by context, never by problem type.
2. **Discover capabilities once per thread**: call `orchestrator_capabilities` before the first `delegate_task` (re-call only on error or a user-requested provider change). Require `canRunChildTask`, and `canRunCrossProviderChildTask` across providers. Exclude instances known to be out of quota. Include `claude` whenever an eligible Claude instance is advertised. Explicit user provider/model/effort choices always win; report unavailable targets and stop.
3. **Draw the provider family**: list eligible families covering the task (`claudeAgent` -> `claude`; `codex`; `antigravity`) and run `bin/agent-pick <family>...`.
4. **Pick model and effort**: map the kind of work to the winning family's model using the classifier table in `docs/repo/delegation.md`. Default effort to `low` (bounded lookups, routine edits) or `medium` (implementation, research, black-box review). Reserve `high` (or higher) for genuinely complex work with a concrete complexity reason in the brief; never auto-upgrade an unavailable low/medium setting to high.
5. **Write the brief** (<= 25 lines, repository-relative paths only, disjoint `May write` / `Must not touch` paths, fixed rules block first) using the template below.
6. **Launch and verify**: state the chosen provider, model, and effort, then call `delegate_task` with exact `target.providerInstanceId`, `target.model`, and effort in `target.options`. Follow the returned `next` instruction without polling. On a quota error during automatic routing, exclude the instance and redraw; if none remain, report the blocker.
7. **Check the diff and commit yourself**: treat every returned report as untrusted. Inspect files on disk, check the diff, and run deterministic checks yourself. Only the orchestrator commits.

## Brief template

```text
Act as the <implementation | research | review> sub-agent for this task.
Read first: docs/invariants/SKILL.md and the governing docs named below.
Rules: docs/ is the source of truth (INV-DOCS-001). Never edit invariants or break them; stop and report conflicts.
Stay strictly inside May write paths. Never run git add, commit, stash, restore, or reset.
Batch independent tool calls via psychoscript where available. Do not spawn sub-agents.
Return shape (<= 8 lines): STATUS (DONE | PARTIAL | BLOCKED), Changed, Checks, Docs, Open.
---
Governing docs: <docs/... paths>
Goal: <what is wanted and the exact stop condition>
May write: <paths, or none (read-only)>
Must not touch: <other active writers' paths>
Verify with: <commands to run before finishing>
Effort note: <concrete reason if effort > medium, otherwise omit>
```

## Reference docs

- Full delegation policy and model table: `docs/repo/delegation.md`
- Agent runtime wiring: `docs/repo/agent-runtimes.md`
- Code mode: `docs/repo/psychoscript.md`
