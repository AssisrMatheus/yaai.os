---
description: Command reference, arguments, exit codes and sample output for the agent-pick entry point.
tags: [tooling, delegation]
---

# agent-pick

Picks one eligible agent family uniformly at random using `/dev/urandom` rejection sampling. Delegation rules and when
to run it live in [[repo/delegation]].

## Synopsis

```sh
bin/agent-pick <codex|antigravity|claude> ...
```

## Commands

| Command                      | Does                                                                                                                                          |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `bin/agent-pick <family>...` | Deduplicate the given eligible families, pick one uniformly at random, print it on stdout, and print the effort-assessment reminder on stderr |

## Options

| Argument / option | Default  | Meaning                                                                                                    |
| ----------------- | -------- | ---------------------------------------------------------------------------------------------------------- |
| `<family>...`     | required | One or more eligible agent families: `codex`, `antigravity`, or `claude`. Duplicate arguments are ignored. |

## Exit codes

- `0`: Picked one family; stdout holds the chosen family name and stderr holds the effort reminder.
- `2`: Missing or unknown family argument; stderr prints the usage line.
- `1`: Failed to read random bytes from `/dev/urandom`.

## Output

```text
claude
Assess the task and prefer low/medium effort. Explain higher effort in the brief for highly complex work.
```

## Covers

- `bin/agent-pick`
