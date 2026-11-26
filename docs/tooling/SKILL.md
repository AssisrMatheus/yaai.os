---
name: yaai-tooling
description: Every reusable repository tool under tooling/ and bin/, with commands, options and exit codes. Use when about to run a repeatable command or one-off script, looking for an existing tool, or adding or changing a tool.
tags: [tooling, index]
---

# Tooling

Every reusable CLI under `tooling/` and entry point under `bin/`. Prefer a tool over a one-off `python`, `bash` or
`bun -e` call when a task repeats or its flags would need rediscovering; check this table first and extend an existing
tool before adding a near-duplicate (INV-TOOLING-003, INV-TOOLING-004, INV-TOOLING-005). Register every tool here and
in its `.tool.md` page in the same commit (INV-TOOLING-006), and give every `tooling/` workspace a human `README.md`
(INV-TOOLING-007). CLI output and exit-code rules live in [[best-practices/tooling-cli.rules]].

## Which file to read

| Tool | Package | Language | Does | Read |
| --- | --- | --- | --- | --- |
| `doc-links` | `@yaai/doc-links` | Rust | Checks code-to-doc links, doc kinds, tool registration and commit-msg drift; lists refs and coverage gaps; injects edges into graphify | [[tooling/doc-links.tool]] |
| `agent-pick` | none | POSIX sh | Picks one eligible agent family (`codex`, `antigravity`, `claude`) uniformly at random for delegation | [[tooling/agent-pick.tool]] |
