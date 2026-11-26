---
description: Supported operating system and shell environment invariants.
tags: [invariants, platform]
---

# Platform invariants

## INV-PLATFORM-001: Unix only: macOS, Linux, WSL

Active · Decided: 2026-11-26, owner

This repository targets Unix environments only: macOS, Linux (Debian, Arch, and other distributions), and WSL under Windows. Native Windows is unsupported; scripts, hooks, and tooling may rely on POSIX `/bin/sh` and standard Unix semantics without adding native Windows code paths.

Explained in [[repo/platforms]] and [[adr/0003-unix-only]].
