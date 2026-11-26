---
description: Supported operating system and shell environment invariants.
tags: [invariants, platform, old]
superseded_by: "[[invariants/platform.invariants]]"
commit: 55311a3
---

# Platform invariants: INV-PLATFORM-001

**Rule as it read:** Unix only: macOS, Linux, WSL

Active · Decided: 2026-11-26, owner

This repository targets Unix environments only: macOS, Linux (Debian, Arch, and other distributions), and WSL under Windows. Native Windows is unsupported; scripts, hooks, and tooling may rely on POSIX `/bin/sh` and standard Unix semantics without adding native Windows code paths.
