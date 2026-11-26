---
description: Supported operating system and shell environment invariants.
tags: [invariants, platform]
---

# Platform invariants

## INV-PLATFORM-001: The development environment is Unix only: macOS, Linux, WSL

Active · Decided: 2026-11-26, owner

Working on this repository is supported on Unix environments only: macOS, Linux (Debian, Arch, and other distributions), and WSL under Windows. Native Windows is unsupported as a development environment; scripts, hooks, git hooks, `bin/` entry points and everything under `tooling/` may rely on POSIX `/bin/sh` and standard Unix semantics without adding native Windows code paths.

This rule covers development only. It does not limit where the product runs: the app is meant to support native Windows one day. Code that ships in an app under `apps/`, or in a package an app uses, must not assume Unix because of this invariant.

Explained in [[repo/platforms]] and [[adr/0003-unix-only.adr]].
