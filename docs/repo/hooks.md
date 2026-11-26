---
description: Every agent hook, its event and matcher, what it enforces, its exit codes, the shell policy, and how to test it.
tags: [repo, hooks, agents]
---

# Agent hooks

One implementation, `.agents/hooks/runtime.ts`, serves every runtime. Each runtime's config calls it through bun with
a client name and a mode (`start`, `pre`, `post`, `stop`); see [[repo/agent-runtimes]] for the wiring. Rules with no
exception live here as code, not prose ([[repo/writing]]).

## Exit codes

Source: [Claude Code hooks, exit codes](https://code.claude.com/docs/en/hooks.md#exit-code-output).

| Code | Meaning                                                                                                                               |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------- |
| 0    | Pass. Invalid JSON input, a crash of the hook itself, and a failing optional tool (rtk, graphify, formatter not installed) also pass. |
| 2    | Block. Exactly one line on stderr tells the agent what to do.                                                                         |

A hook never blocks because of its own failure. Antigravity, Pi and OMP receive the same decision through their native
envelope (`deny` or `continue` with a reason, `block` with a reason) instead of exit 2.

## Events

| Event                     | Matcher                                                                                                                              | What it does                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| SessionStart              | all                                                                                                                                  | Injects two lines: open `AGENTS.md` and `docs/SKILL.md`, and whether `graphify-out/graph.json` exists. After a compact it resets the graph guard budget. Records the starting HEAD when `bin/doc-links` exists.                                                                                                                                                    |
| PreToolUse (shell)        | Bash (Codex: Bash, apply_patch; Antigravity: run_command)                                                                            | Applies the [[repo/hooks#Shell policy]]. Passes the call to `graphify hook-guard search` for graph advice, then rewrites foreground commands through `rtk hook claude`, except whole `git diff` and `git show`.                                                                                                                                                    |
| PreToolUse (read, search) | Read, Glob, Grep (Antigravity: view_file, list_dir, find_by_name, grep_search)                                                       | Forwards graph advice when a graph exists.                                                                                                                                                                                                                                                                                                                         |
| PreToolUse (edit)         | Edit, Write, MultiEdit, NotebookEdit; Codex apply_patch; Antigravity write_to_file, replace_file_content, multi_replace_file_content | Blocks any path that resolves into `vendor/`, including through symlinks and patch moves or deletes.                                                                                                                                                                                                                                                               |
| PostToolUse (edit)        | the edit tools above                                                                                                                 | Formats the file (rustfmt for `.rs`; oxfmt for JS, TS, JSON, Markdown, YAML, TOML, CSS and HTML; a missing formatter is silent), runs `bin/doc-links check <file>` for files under `docs/`, then runs `bin/lint -- <files>` on the edited Rust, JS and TS files ([[repo/lint-rules]]). Lint findings (exit 2) block; a failure of the lint runner itself does not. |
| PostToolUse (output)      | Claude: Bash, Monitor, Grep, Glob, Read, WebFetch, WebSearch                                                                         | Passes the result to `rtk hook claude` and returns its condensed output when it has one ([[repo/rtk]]).                                                                                                                                                                                                                                                            |
| Stop, SubagentStop        | all                                                                                                                                  | Runs `bin/doc-links check` when the checkout is dirty or HEAD moved since the session started; skips a clean unchanged one. A failing check keeps the agent working, once: a stop that already follows a blocked stop (`stop_hook_active`) passes, so findings in files another agent is writing cannot loop the session.                                          |

Hooks do not execute agent commands themselves. They forward events and apply rules.

## Shell policy

The policy reads literal commands, including `env`/`sudo` prefixes, `sh -c` strings, and compound commands.

| Rule                                                                                                                                            | Agent is told                                                  |
| ----------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| Packages go through pnpm: `npm`, `npx`, `yarn`, `bunx`, `bun add/install/remove/update/x`                                                       | Use pnpm (`pnpm exec`, `pnpm dlx` for executables).            |
| TypeScript runs with bun: `tsx`, `ts-node`, `node file.ts`, `deno run`, `pnpm exec tsx`                                                         | Run TypeScript with `bun <file.ts>`.                           |
| No `git --no-verify`, no `git commit -n`                                                                                                        | Run Git with verification enabled and fix the reported checks. |
| No rtk setup command and no graphify agent-install command                                                                                      | Use the repository rtk pipeline and hooks.                     |
| `vendor/` is read-only: redirects, `mv cp rm mkdir touch tee chmod sed -i ...`, `-o`/`--output` targets, git state changes with `-C vendor/...` | Write to a project-owned path outside `vendor/`.               |

The checks cover observable native commands and edits. An arbitrary script, a dynamic shell expansion or a direct host
API can still write elsewhere; only filesystem permissions give an absolute `vendor/` guarantee.

## Per runtime

| Runtime     | Shell and edit checks                      | Rewrite and output                                                                        | Stop gate                                                 |
| ----------- | ------------------------------------------ | ----------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| Claude Code | PreToolUse, exit 2                         | `updatedInput`; PostToolUse `updatedToolOutput`                                           | Stop, SubagentStop                                        |
| Codex       | PreToolUse on Bash and apply_patch, exit 2 | `updatedInput` with allow; no output rewrite                                              | Stop, SubagentStop                                        |
| Antigravity | PreToolUse, `deny` with reason             | `overwrite.CommandLine`; PostToolUse emits the line, visible output depends on the client | Stop with `continue`; no SubagentStop                     |
| Pi          | `tool_call` returns `block`                | input mutated in place                                                                    | `agent_before_settle` continues                           |
| Oh My Pi    | `tool_call` returns `block`                | input returned                                                                            | `session_stop` blocks; subagent `agent_end` only notifies |

## Test

```sh
bun test .agents/hooks
```

`bunfig.toml` sets the test root. Done when the command exits 0. `t3.json` offers the same command as the "Test
agent hooks" action in T3 Code. Add a case in `.agents/hooks/hooks.test.ts` with every rule you add.

## Covers

- `.agents/hooks/runtime.ts`
- `.agents/hooks/checks.ts`
- `.agents/hooks/policy.ts`
- `.agents/hooks/pi.ts`
- `.agents/hooks/hooks.test.ts`
- `.agents/hooks.json`
- `.claude/settings.json`
- `.codex/hooks.json`
- `bunfig.toml`

## References

- [Claude Code hooks](https://code.claude.com/docs/en/hooks.md)
- [Codex hooks](https://learn.chatgpt.com/docs/hooks.md)
- [Antigravity hooks](https://antigravity.google/docs/hooks/)
- [Oh My Pi extensions](https://github.com/can1357/oh-my-pi/blob/main/docs/extensions.md)
- [Pi extensions](https://docs.pi.dev/latest/extensions)
- [Using hooks to enforce the right CLI](https://www.aihero.dev/how-to-use-claude-code-hooks-to-enforce-the-right-cli)
