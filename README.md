# yaai.os

Yet another AI OS. A home OS: the homepage of your media center.

No ads. No suggested content you didn't ask for.

## Status

Early. The repository, its docs and its agent set-up exist. The app does not yet.

## Requirements

- To work on this repo: macOS, Linux, or WSL. Native Windows is not supported for development.
  (The app itself will support Windows one day.)
- Node 24+, pnpm, bun, Rust.
- Optional, for AI agents: [rtk](https://github.com/rtk-ai/rtk) and [graphify](https://github.com/Graphify-Labs/graphify).

## Start

```sh
pnpm install
pnpm lint
pnpm test
```

If you use rtk and graphify, run these once:

```sh
rtk trust --yes
GRAPHIFY_BACKEND=none graphify update .
```

## Layout

| Folder | What is in it |
| --- | --- |
| `apps/` | Things that ship |
| `packages/` | Shared code |
| `tooling/` | Command-line tools for working on the repo |
| `bin/` | Short scripts that run those tools |
| `vendor/` | Outside code, read-only, not committed |
| `docs/` | The rules, decisions and specs. Open it in Obsidian. |
| `.agents/` | Skills and hooks for AI agents |

## How we work

Docs first. A change starts as a page in `docs/`, then becomes code.

Want to contribute? Read [CONTRIBUTING.md](CONTRIBUTING.md).

Working with an AI agent? It reads `AGENTS.md`.
