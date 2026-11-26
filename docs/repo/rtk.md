---
description: What rtk does for agents here, how its pipeline is wired, the one-time trust step, and how to check and use it.
tags: [repo, rtk, tooling, agents]
---

# rtk: condensed command output

rtk compresses the output of shell commands (test runs, builds, logs, `git status`) before an agent reads it, and keeps
the original recoverable. It is optional: the hooks skip it silently when the binary is absent (ADR 0008).

## Wiring

- `.rtk/pipeline.toml` defines the pipeline `yaai-os`, which extends the built-in `rtk:posthook`.
- Two passthrough rules keep output whole: workspace `pnpm lint` and any `git diff` or `git show`.
- The PreToolUse hook rewrites a foreground Bash command through `rtk hook claude`; the PostToolUse hook condenses
  results from Bash, Monitor, Grep, Glob, Read, WebFetch and WebSearch on Claude Code ([[repo/hooks]]). Background
  commands are not rewritten. The adapter also preserves whole diffs before and after the tool.
- Hooks forward the JSON protocol to rtk. They never run an agent command themselves.
- Version tested: 1.2.0. Tools under `tooling/` print compact output that rtk compresses well (INV-TOOLING-001).

## One-time steps after cloning

1. Install rtk ([[repo/platforms#First steps after cloning]]).
2. Read `.rtk/pipeline.toml`.
3. From the repository root run `rtk trust --yes`. Trust is machine-local; repeat it after every edit of the pipeline.

Never run rtk's own agent-setup command: it writes user-level config, and the shell policy blocks it. Wiring is by
hand in the repository.

## How an agent uses it

- Run commands normally. The hook rewrites them; do not prefix `rtk` for everything.
- Treat condensed output as complete. A line like `[+39 hidden: rtk recall b0612bbe854c7909]` means rtk dropped
  detail; run `rtk recall <hash>` only when the omitted part matters to the task.
- Prefix `rtk` by hand only for a command the hook does not cover (background commands), or `rtk bypass <command>` for
  raw output.
- Edit `.rtk/pipeline.toml` only when a command's output must stay whole; add a passthrough rule and re-trust.

## Check that it works

```sh
rtk pipeline validate
rtk pipeline test
rtk pipeline explain --tool Bash --command 'git diff'
rtk hook check git status
```

Expect the resolved pipeline `yaai-os`, the `whole-diffs` passthrough for `git diff`, and a rewrite to `rtk git
status`.

## Failure modes

| Symptom | Cause and fix |
| --- | --- |
| Output is never condensed | rtk missing or pipeline untrusted: install it, then `rtk trust --yes`. |
| A command's output is cut and you need it | `rtk recall <hash>` or `rtk bypass <command>`. |
| Pipeline validate fails after an edit | Fix the TOML, then trust again. |

## Covers

- `.rtk/pipeline.toml`

## References

- [rtk](https://github.com/rtk-ai/rtk)
