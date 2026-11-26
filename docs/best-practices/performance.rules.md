---
description: How speed, size and load are prioritised, budgeted and measured, and which levers apply to user-facing surfaces.
tags: [best-practices, performance]
---

# Performance

Language-level hot-path rules are in [[best-practices/rust-ownership.rules]]; tool start-up budgets are in
[[best-practices/tooling-cli.rules]].

## Priorities and measurement

### BP-PERF-001: Keep results before speed

Correct results, invariants and the order of effects come first, performance second, idiom and brevity third; speed never buys a different result.
Why: a fast wrong answer is a bug.
Check: review

### BP-PERF-002: Back every speed claim with a measurement

A change made for speed records before and after figures on the same host, the same input and the shipped profile, with the command used.
Why: a change "for speed" without a figure is a style change.
Check: review

### BP-PERF-003: Name the limiting factor

State why the result is X and not half of X: which resource limits it.
Why: without the limit, the next optimisation is a guess.
Check: review

### BP-PERF-004: Set a budget per path

Each user-facing path and hot path has a written budget (time, allocations, peak memory, size) measured on a release build with representative input.
Why: a budget turns "fast enough" into a check.
Check: review

### BP-PERF-005: Keep a regression case for expensive paths

An expensive path keeps a benchmark or counting test that fails when it regresses past its budget.
Why: performance regresses silently otherwise.
Check: test

### BP-PERF-006: Let measured speed override a lint

When a lint's preferred form costs measured time on a hot path, take the faster form with `#[expect(<lint>, reason = "measured: <figure, where recorded>")]` on the smallest item.
Why: performance outranks idiom, but only with evidence.
Check: review

### BP-PERF-007: Configure the release profile once

Set the release profile (optimisation level, LTO, codegen units) at the workspace; crates do not override it, and changing it needs a measurement.
Why: one profile means one set of numbers to compare.
Check: review

### BP-PERF-008: Judge speed on a release build

Measure and judge performance on a release or production build, never a development build.
Why: development builds carry diagnostics and skip optimisation.
Check: review

### BP-PERF-009: Judge user-facing speed on field data

Judge a user-facing surface on measurements from real use (75th percentile) over lab scores.
Why: lab runs miss the devices and networks people use.
Check: review

## User-facing surfaces

### BP-PERF-010: Meet the interaction budgets

On web surfaces, keep the 75th percentile at largest contentful paint ≤ 2.5 s, interaction to next paint ≤ 200 ms and cumulative layout shift ≤ 0.1.
Why: these are the thresholds where a surface feels fast and stable.
Check: review

### BP-PERF-011: Measure bundle cost before adding a heavy dependency

Compare the shipped client code before and after adding a heavy client dependency or moving code to the client, and record the change.
Why: client code is paid by every user on every load.
Check: review

### BP-PERF-012: Load heavy client features on demand

Load heavy client-only features (players, maps, rich editors) behind the interaction or below the first screen; never use lazy loading to hide code that belongs on the trusted side.
Why: the first screen loads only what it shows.
Check: review

### BP-PERF-013: Load scripts only where used

Load a script on the screen that needs it, with an explicit loading strategy; nothing render-blocking before the first paint.
Why: global scripts tax every screen.
Check: review

### BP-PERF-014: Virtualise long lists when profiling shows cost

Render only the visible window of a long list once profiling shows the list is a cost.
Why: thousands of off-screen nodes cost layout and memory.
Check: review

### BP-PERF-015: Pause work that is off screen

Pause loops, animations, timers and polling while their surface is hidden or off screen.
Why: invisible work drains battery and competes with visible work.
Check: review

### BP-PERF-016: Keep long work out of input handlers

Break up long synchronous tasks and move heavy work out of event handlers.
Why: the next interaction waits for the main thread.
Check: review

### BP-PERF-017: Memoise only with a reason

Add manual memoisation only where a measurement or the compiler's bail-out shows a need, with a comment saying which.
Why: unneeded memoisation hides dependencies and goes stale.
Check: review

### BP-PERF-018: Tune imports only after measuring

Change import optimisation or tree-shaking settings only after the bundle analysis shows a package costs too much.
Why: guessing adds configuration without gain.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (hot paths, budgets, benchmarks and client bundles)
- `Cargo.toml`
