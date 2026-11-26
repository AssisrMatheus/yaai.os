---
description: Structured logging invariants across apps, packages, and tooling.
tags: [invariants, logging]
---

# Logging invariants

## INV-LOGGING-001: Apps, packages and tooling log extensively to the repository logging standard

Active · Decided: 2026-11-26, owner

Every app, package, and tool emits structured, scoped diagnostic events to stderr with UTC millisecond timestamps, stable dot-separated scopes, bounded single-line rendering, and secret redaction before any sink. Stdout remains reserved for command output.

Explained in [[best-practices/logging.rules]].
