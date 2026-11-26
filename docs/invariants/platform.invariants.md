---
description: Supported operating system and shell environment invariants.
tags: [invariants, platform]
---

# Platform invariants

## INV-PLATFORM-001: The development environment is Unix only: macOS, Linux, WSL

Active · Decided: 2026-11-26, owner

Working on this repository is supported on Unix environments only: macOS, Linux (Debian, Arch, and other distributions), and WSL under Windows. Native Windows is unsupported as a development environment; scripts, hooks, git hooks, `bin/` entry points and everything under `tooling/` may rely on POSIX `/bin/sh` and standard Unix semantics without adding native Windows code paths.

This rule covers development only. It says nothing about where the product runs: the platforms of the apps under `apps/` and of the packages they ship are undecided and may include native Windows. Code that ships in an app must not assume Unix because of this invariant.

Explained in [[repo/platforms]] and [[adr/0003-unix-only.adr]].
