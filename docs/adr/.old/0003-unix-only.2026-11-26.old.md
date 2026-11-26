---
description: Decide to target Unix environments only (macOS, Linux, WSL) and leave native Windows unsupported.
tags: [adr, platform, old]
superseded_by: '[[adr/0003-unix-only.adr]]'
commit: 55311a3
---

# Unix only

On 2026-11-26 the owner decided to support Unix environments only — macOS, Linux (Debian, Arch, and other distributions), and WSL under Windows — and leave native Windows unsupported (`INV-PLATFORM-001`, [[repo/platforms]]). Relying on POSIX `/bin/sh`, symlinks, and standard Unix tools removes cross-shell compatibility work from every hook, wrapper, and CLI.

## Considered options

- **Native Windows support alongside Unix**: rejected because maintaining PowerShell/batch equivalents, non-symlink fallbacks, and Windows path handling slows every tooling and hook change for an unsupported target.

## Consequences

- Wrappers under `bin/` and hooks may assume POSIX shell and standard Unix semantics.
