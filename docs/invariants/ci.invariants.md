---
description: The initial warm CI latency budget and the four checks that protect it.
tags: [invariants, ci]
---

# CI invariants

## INV-CI-001: Keep the required check fast

Active · Decided: 2026-11-26, owner

The initial budget is 12 seconds of command wall time for the slowest warm job on the local development baseline. Measure after installing dependencies and restoring build and task caches. Track hosted setup, queue and cold build times separately in [[repo/ci]].

1. Run Clippy wherever applicable and oxlint on its supported remaining inputs, using defaults plus repository rules.
2. Balance test shards by recorded duration, longest first, with the median for new units.
3. On pull requests, test changed workspaces and their dependents, and check affected root inputs. Shared config reaches all workspaces. Pushes to main run all tests.
4. Lint, typecheck, build and test start independently. Their aggregate is the sole required check.
