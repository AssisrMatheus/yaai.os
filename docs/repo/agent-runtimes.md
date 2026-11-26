---
description: How one .agents/ source reaches Claude Code, Codex, Antigravity, Oh My Pi, Pi and T3 Code, and how to add a skill, hook or role.
tags: [repo, agents, runtimes]
---

# Agent runtimes

`.agents/` is the single source of skills, hooks and agent roles (INV-AGENTS-001, ADR 0007). Runtime folders hold
symlinks or thin adapters that point at it; they never hold a second copy.

```text
.agents/skills/<name>/SKILL.md   project skills
.agents/agents/                  agent roles (reserved with .gitkeep)
.agents/hooks/                   shared hook code: runtime.ts checks.ts policy.ts pi.ts hooks.test.ts
.agents/hooks.json               Antigravity hook bundle
.agents/mcp_config.json          symlink to ../.mcp.json
```

## Runtime map

| Runtime                  | Instructions                                     | Skills                               | Hooks config                                                 | MCP config                                      | Roles                                |
| ------------------------ | ------------------------------------------------ | ------------------------------------ | ------------------------------------------------------------ | ----------------------------------------------- | ------------------------------------ |
| Claude Code              | `CLAUDE.md` -> `AGENTS.md`                       | `.claude/skills` -> `.agents/skills` | `.claude/settings.json`                                      | `.mcp.json`, enabled in `.claude/settings.json` | `.claude/agents` -> `.agents/agents` |
| Codex (standalone or T3) | `AGENTS.md` chain from root to working directory | `.agents/skills`                     | `.codex/hooks.json`                                          | `.codex/config.toml`                            | `.codex/agents` -> `.agents/agents`  |
| Antigravity              | `AGENTS.md`                                      | `.agents/skills`                     | `.agents/hooks.json` (bundle `yaai-os`)                      | `.agents/mcp_config.json` -> `.mcp.json`        | `.agents/agents`                     |
| Oh My Pi                 | `AGENTS.md`                                      | `.agents/skills`                     | `.omp/extensions/repository.ts`                              | root `.mcp.json`                                | `.omp/agents` -> `.agents/agents`    |
| Pi                       | `AGENTS.md`                                      | `.agents/skills`                     | `.pi/extensions/repository.ts`                               | `.pi/mcp.json` -> `.mcp.json`                   | none configured                      |
| T3 Code                  | the selected provider reads its own files        | same as provider                     | none: `t3.json` only adds a manual "Test agent hooks" action | provider's                                      | provider's                           |

Every hook entry runs `.agents/hooks/runtime.ts` through bun, so the policy is written once ([[repo/hooks]]).
`.codex/config.toml` also enables Codex native code mode, `.omp/config.yml` enables OMP `eval.js`, and
`.pi/settings.json` enables Pi codemode ([[repo/psychoscript#Native fallbacks]]).

## Trust steps

Each runtime requires one manual step after cloning. Project files do nothing until it is done.

| Runtime     | Step                                                                                           |
| ----------- | ---------------------------------------------------------------------------------------------- |
| Claude Code | Accept the project's hooks and the `graphify` MCP server when prompted.                        |
| Codex       | Trust the project; review the command-hook fingerprints in `/hooks`; restart after any change. |
| Antigravity | Open the project; `/hooks` in its CLI lists the registrations.                                 |
| Oh My Pi    | Open the project; it loads `.omp/extensions/repository.ts`.                                    |
| Pi          | Run `/trust` for the project resources.                                                        |
| T3 Code     | Nothing of its own; trust applies in the provider that runs the thread.                        |

## Add something once, every runtime gets it

| Add           | Do                                                                                                                                                               | Done when                                                           |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| A skill       | Create `.agents/skills/<name>/SKILL.md` (`name`, `description` = what it is and when to use it). Procedure only; facts live in docs it links ([[repo/writing]]). | Every runtime lists it; no file is added outside `.agents/skills/`. |
| A hook rule   | Edit `.agents/hooks/policy.ts` or `checks.ts` and add a case to `hooks.test.ts`. No runtime file changes.                                                        | `bun test .agents/hooks` passes.                                    |
| A hook event  | Add it to `runtime.ts` first, then register it in each hook config above that supports it.                                                                       | The test passes and each config names the event.                    |
| A role        | Put the file in `.agents/agents/`. Claude uses Markdown roles, Codex uses TOML roles, so each loader's own format must be satisfied.                             | Each runtime's role listing shows it.                               |
| An MCP server | Add it to `.mcp.json` and `.codex/config.toml`. T3 inherits the provider's config.                                                                               | `graphify` shows how both are kept in step ([[repo/graphify]]).     |

Do not copy a definition into a runtime folder (INV-AGENTS-001). The only third-party skill here is
`.agents/skills/graphify/`; other skills are project skills.

## What was not verified live

Verified: `bun test .agents/hooks`, sampled native commands from a subdirectory, a first graph build, the graphify MCP
`initialize`, `tools/list` and `graph_stats`, Codex features and MCP config in a disposable trusted home, and Pi
recognising `.pi/mcp.json`. Not verified: live authenticated model turns, T3 delegated provider sessions, the
Antigravity executable, macOS and WSL. After granting trust, confirm that dispatch runs in the runtime you use.

## Covers

- `.agents/hooks.json`
- `.agents/mcp_config.json`
- `.claude/settings.json`
- `.codex/config.toml`
- `.codex/hooks.json`
- `.omp/config.yml`
- `.omp/extensions/repository.ts`
- `.pi/settings.json`
- `.pi/extensions/repository.ts`
- `.mcp.json`
- `t3.json`

## References

- [Claude Code hooks](https://code.claude.com/docs/en/hooks.md)
- [Codex hooks](https://learn.chatgpt.com/docs/hooks.md)
- [Codex AGENTS.md](https://learn.chatgpt.com/docs/agent-configuration/agents-md.md)
- [Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents.md)
- [Codex skills](https://learn.chatgpt.com/docs/build-skills.md)
- [Antigravity hooks](https://antigravity.google/docs/hooks/)
- [Antigravity rules](https://antigravity.google/docs/rules/)
- [Antigravity customization discovery](https://antigravity.google/docs/customization-discovery/)
- [Antigravity subagents](https://antigravity.google/docs/subagents/)
- [Antigravity MCP](https://antigravity.google/docs/mcp/)
- [Oh My Pi extensions](https://github.com/can1357/oh-my-pi/blob/main/docs/extensions.md)
- [Oh My Pi task agent discovery](https://github.com/can1357/oh-my-pi/blob/main/docs/task-agent-discovery.md)
- [Oh My Pi config](https://github.com/can1357/oh-my-pi/blob/main/docs/config-usage.md)
- [Oh My Pi skills](https://github.com/can1357/oh-my-pi/blob/main/docs/skills.md)
- [Pi extensions](https://docs.pi.dev/latest/extensions)
- [Pi MCP](https://docs.pi.dev/latest/mcp)
- [Pi skills](https://docs.pi.dev/latest/skills)
- [Pi security](https://docs.pi.dev/latest/security)
- [T3 Code project settings](https://github.com/pingdotgg/t3code/blob/main/docs/user/project-settings.md)
- [T3 Code t3.json](https://github.com/pingdotgg/t3code/blob/main/docs/user/t3-json.md)
- [T3 Code providers](https://github.com/pingdotgg/t3code/blob/main/docs/internals/03_providers.md)
- [Writing for agents](https://raw.githubusercontent.com/mattpocock/skills/main/skills/productivity/writing-for-agents/SKILL.md)
