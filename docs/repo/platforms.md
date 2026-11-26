---
description: Which operating systems you can develop this repository on, and the tools needed to work in it.
tags: [repo, platforms]
---

# Platforms and prerequisites

INV-PLATFORM-001, ADR 0003: the development environment is Unix only. This page is about working on the repository. Where the
product runs is undecided: the app may support native Windows, so shipped code does not assume Unix.

| Develop on | Not supported for development |
| --- | --- |
| macOS | Native Windows |
| Linux (Debian, Arch, others) | |
| WSL under Windows | |

Scripts, hooks and tooling may rely on POSIX `/bin/sh`, symlinks (`.claude/skills` and `.agents/mcp_config.json` are
symlinks) and Unix paths. No native Windows code path is written or tested for them. Only Linux was exercised during set-up;
macOS and WSL are unverified.

## Prerequisites

| Tool | Needed for | Version |
| --- | --- | --- |
| node | pnpm, husky, lint-staged, commitlint, turbo | `>= 24` (`engines` in `package.json`, `.nvmrc`) |
| pnpm | every package operation | `packageManager` in `package.json` |
| bun | agent hooks (`.agents/hooks/*.ts`) and TypeScript tooling | hooks do not run without it |
| Rust toolchain | tooling in `tooling/*` | pinned by `rust-toolchain.toml` (rustup installs it) |
| rtk | condensed command output, optional ([[repo/rtk]]) | 1.2.0 tested |
| graphify | code and doc graph, optional ([[repo/graphify]]) | 0.9.111 tested |

The hooks skip optional tools silently when they are absent.

## First steps after cloning

1. `pnpm install` (the `prepare` script activates the husky hooks).
2. Build the tools agents call, for example `pnpm exec turbo run build --filter=@yaai/doc-links` ([[repo/doc-links]]).
3. Optional rtk: install, then trust the pipeline ([[repo/rtk]]).
4. Optional graphify: install, then the first build ([[repo/graphify]]).
5. Open the project in your agent runtime and grant trust ([[repo/agent-runtimes#Trust steps]]).

Install commands for the optional tools, from their READMEs:

```sh
brew install rtk                                                                             # macOS, or Linux with Homebrew
curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh   # any Unix
uv tool install 'graphifyy[mcp]'                                                             # or: pipx install 'graphifyy[mcp]'
```

Run only the install commands above. The tools' own agent-setup commands write user-level config, and the hooks block
them ([[repo/hooks#Shell policy]]).

## References

- [rtk](https://github.com/rtk-ai/rtk)
- [Graphify](https://github.com/Graphify-Labs/graphify)
