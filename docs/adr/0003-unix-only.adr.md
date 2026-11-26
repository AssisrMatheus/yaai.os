---
description: Decide that developing in this repository is Unix only (macOS, Linux, WSL), without deciding where the product runs.
tags: [adr, platform]
status: accepted
---

# Unix-only development environment

On 2026-11-26 the owner decided that this repository is developed on Unix environments only — macOS, Linux (Debian, Arch, and other distributions), and WSL under Windows — and that native Windows is unsupported for development (`INV-PLATFORM-001`, [[repo/platforms]]). Relying on POSIX `/bin/sh`, symlinks, and standard Unix tools removes cross-shell compatibility work from every hook, wrapper, and CLI. The decision is about development only: the app that this repository builds is meant to support native Windows one day, and its target platforms are a separate decision.

## Considered options

- **Native Windows as a development environment alongside Unix**: rejected because maintaining PowerShell/batch equivalents, non-symlink fallbacks, and Windows path handling slows every tooling and hook change.

## Consequences

- Wrappers under `bin/`, hooks and tools under `tooling/` may assume POSIX shell and standard Unix semantics.
- Code that ships in an app or in a package an app uses must not assume Unix on the strength of this decision.
