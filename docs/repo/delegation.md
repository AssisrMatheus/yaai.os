---
description: When to delegate to sub-agents, how to pick providers and models in T3 Code, and how to write and verify sub-agent briefs.
tags: [repo, agents, delegation]
---

# Sub-agent delegation

## Covers

- `bin/agent-pick`

## Main loop first

A single session with focused docs and skills beats a multi-agent relay on quality, cost, and latency whenever the work shares context. Keep work in the main session by default:

- Chained edits on the same files or data where one step feeds the next.
- Work that requires a long brief going out and a lossy summary coming back.
- Small tasks cheaper to finish directly than to brief and verify.

Delegate to a sub-agent only when one of these holds:

1. **Parallel independent slices**: self-contained pieces with disjoint write paths and a clear contract.
2. **Black-box verification**: fresh eyes judging a diff against governing docs and deterministic check outputs without seeing the implementer's reasoning.
3. **Context isolation**: a broad lookup, research sweep, or verbose investigation that would flood the main context.

**Split by context, never by problem type.** One sub-agent owns one self-contained slice end to end rather than passing partial work through a chain of specialists.

## Ownership, isolation and trust

- **Targeted context**: the orchestrator gives a sub-agent only the references it needs, the governing rules, and a clear description of what is wanted. Let sub-agents write files directly inside their granted paths rather than copying output through chat.
- **Disjoint write paths**: every parallel writer gets explicit `May write` and `Must not touch` paths in a shared checkout, or `isolation: "worktree"` when separate checkouts are needed.
- **Untrusted reports**: a sub-agent's reply is a claim, not proof. Check the actual diff, files on disk, and deterministic checks before relying on it.
- **Orchestrator commits**: sub-agents never run `git add`, `git commit`, `git stash`, `git restore`, or `git reset`. The orchestrator reviews and commits.
- **One return shape** (at most 8 lines):

```text
STATUS: DONE | PARTIAL - <what remains> | BLOCKED - <what is needed>
Changed: <paths, or none>
Checks: <command> -> PASS|FAIL, one per line
Docs: <pages updated> | unchanged: <why>
Open: <questions, or none>
```

## T3 Code dispatch procedure

Follow T3 Code's orchestration rules ([[repo/agent-runtimes]]). For cross-provider work, custom settings, background execution, or isolated worktrees — and whenever selecting across provider families — dispatch via `delegate_task` with the exact provider instance, model, and effort option:

1. **Discover capabilities once per thread**: call `orchestrator_capabilities` before the first `delegate_task` (re-call only after an error or a user-requested provider change). Require `canRunChildTask`, and `canRunCrossProviderChildTask` across providers. Exclude instances known to be out of quota; when no quota signal is reported, usage is unknown until a launch hits a limit. Include `claude` automatically whenever discovery reports an eligible Claude instance. Explicit user requests for provider, model, or effort always win; if unavailable, report it and stop rather than switching silently.
2. **Draw the provider family**: list every family whose advertised models cover the scope (`claudeAgent` normalizes to `claude`; `codex` and `antigravity` keep their names) and run:

   ```sh
   bin/agent-pick <family>...
   ```

   `bin/agent-pick` deduplicates families so each eligible family gets one uniform entry regardless of instance count, draws from `/dev/urandom`, prints the winning family to stdout, prints the effort reminder to stderr, and exits `2` on invalid arguments.

3. **Select model and effort**: pick the matching model in the drawn family:

   | Kind of work                                                      | `codex` | `antigravity`        | `claude` |
   | ----------------------------------------------------------------- | ------- | -------------------- | -------- |
   | Bounded lookups, routine edits, coding from a clear example       | Sol     | Neon                 | Sonnet   |
   | Implementation without an example, architecture, black-box review | Astra   | Opus (if advertised) | Opus     |
   | Large orchestration or open-ended research                        | Astra   | —                    | Fable    |

   Assess the actual scope before choosing effort: default to **low** for bounded lookups and routine edits, and **medium** for implementation, research, and black-box review. A role or model family never implies high effort. Use **high** (or higher) only for genuinely complex work with a concrete complexity reason stated in the brief. Never round an unavailable low/medium setting up to high; choose a suitable advertised model/setting or report the limitation.

4. **Launch and handle quota errors**: state the selected provider, model, and effort, then call `delegate_task` with `target.providerInstanceId`, `target.model`, and the advertised effort in `target.options`. Follow the returned `next` instruction without polling. On a quota error during automatic routing, exclude the affected instance and redraw from the remaining eligible families; if none remain, report the blocker.

## Brief template

Keep briefs under 25 lines. Pass repository-relative paths, never pasted file contents. Put the fixed rules block first so parallel dispatches share a prompt prefix, and put task-specific scope after `---`.

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

## References

- [T3 Code orchestrator MCP server](https://github.com/pingdotgg/t3code/blob/76de50bc0d3fdd9fc2c5816de4142fbda1ce8364/docs/orchestration-v2/orchestrator-mcp-server.md)
- [Building agents: When to fold subagents into your main agent loop](https://claude.com/resources/articles/building-agents-when-to-fold-subagents-into-your-main-agent-loop)
