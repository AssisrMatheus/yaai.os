---
description: What to test, where tests live, how to make them deterministic and readable, and what never to do to get a test green.
tags: [best-practices, testing]
---

# Testing

Tests pin promises someone relies on. They run fast enough for every commit, give the same result on every machine
and fail only when such a promise breaks. Hooks and CI are in [[best-practices/ci]].

## What to test

### BP-TEST-001: Name the test after the promise

The test name states the guarantee it pins ("refuses a header with an unknown version").
Why: a failure then reads as a broken guarantee.
Check: review

### BP-TEST-002: Assert through the interface

Assert results, typed failures and observable effects through the module's interface, never its internal state.
Why: the interface is the test surface ([[best-practices/module-design]]).
Check: review

### BP-TEST-003: Write tests that survive refactors

A refactor that keeps behaviour keeps every test green; a test that must change with the implementation tests past the interface.
Why: tests that pin internals cost a rewrite and catch nothing.
Check: review

### BP-TEST-004: Replace shallow tests, do not layer them

When interface-level tests cover a module, delete the older tests of its internal parts.
Why: duplicated coverage doubles the cost of every change.
Check: review

### BP-TEST-005: Pin every fixed bug

A bug fix lands with the test that would have caught it, shown failing on the original code first.
Why: the test proves it distinguishes the fix.
Check: review

### BP-TEST-006: Test the contracts

Test mappers (source-shaped input to domain value, including the source's quirks), services (each answer and each returned error), change handlers (what they flush), views (what a user relies on) and pure helpers.
Why: these are the promises callers depend on.
Check: review

### BP-TEST-007: Test each recovery and each boundary

Cover every typed failure that has a different recovery, and the boundary values of each input.
Why: failures and edges are where code is wrong.
Check: review

### BP-TEST-008: Skip what something else already checks

Do not test the framework or language, pure pass-throughs, types the compiler checks, styling classes or private helpers directly.
Why: such tests break on refactors and prove nothing.
Check: review

### BP-TEST-009: Pin invariants by id

A test that enforces an invariant carries its id in the name (`INV-PRODUCT-001: ...`) ([[repo/doc-links]]).
Why: a failure names the contract it protects, and `doc-links matrix` finds untested invariants.
Check: test

## Where tests live

### BP-TEST-010: Put unit tests beside the code

Rust unit tests sit in `#[cfg(test)] mod tests` in the same file; cross-module tests in the crate's `tests/`. TypeScript tests sit beside the module ([[best-practices/typescript]]). Only cross-cutting suites get their own folder.
Why: a test moves and is reviewed with its code.
Check: review

### BP-TEST-011: Run tests through the workspace task

Run tests with the workspace's `test` task (`pnpm test`, `cargo test --workspace`), never a globally installed runner.
Why: the pinned runner and the workspace setup apply.
Check: review

### BP-TEST-012: Organise suites by question and cost

Split tests into tiers by the question each answers (unit, integration, end-to-end, visual) and run the cheap tiers most often, the expensive ones when their inputs change.
Why: the pre-commit tier stays seconds long.
Check: review

## Doubles and fixtures

### BP-TEST-013: Keep unit tests offline

Unit tests make no network calls and touch no real external service.
Why: offline tests are fast and deterministic.
Check: test

### BP-TEST-014: Prefer fakes that implement the port

Test a service against an in-memory fake that implements its trait or port, not a mock of an import path.
Why: the compiler checks a fake when the port changes; nothing checks a path mock.
Check: review

### BP-TEST-015: Make fixtures fail loudly

A fixture or fake fails on an unknown key or unsupported request instead of returning empty; where a schema exists, the fake answers by executing it.
Why: a missing fixture field must not pass as "empty".
Check: review

### BP-TEST-016: Prove the fake was used

When a suite points code at a fake, assert the fake received requests.
Why: a green run with zero requests may have reached the real service, or nothing.
Check: test

### BP-TEST-017: Install module-wide mocks only in setup

Process-wide mocks and replaced modules are installed once in the suite's setup, never in an individual test file; per-test doubles are passed in.
Why: global mocks leak into later files in scheduling order.
Check: review

### BP-TEST-018: Derive fixture ids from content

Fixture ids are stable values (a hash of the row), never random or time-based.
Why: a failure names the same row on every run.
Check: review

## Determinism

### BP-TEST-019: Fix time, randomness and locale

Inject a fixed clock, a seeded RNG, a fixed locale and timezone, and advance time explicitly for timers and retries.
Why: tests that read the wall clock fail at midnight and in other countries.
Check: test

### BP-TEST-020: Set the whole environment

The suite sets every environment variable the code reads; it never inherits a developer's local environment.
Why: a local `.env.local` must never change a result.
Check: test

### BP-TEST-021: Make tests independent of order

Restore state after each case and keep output order deterministic under concurrency.
Why: order-dependent tests fail one run in ten.
Check: test

### BP-TEST-022: Pin outputs of deterministic engines

For a deterministic engine (a scheduler, a ranking, a converter), compare a fixed input's full output with a committed file; re-pin it only when the change meant to alter it.
Why: any unintended change to the output fails at once.
Check: test

### BP-TEST-023: Make guard tests fail closed

A test that checks files or configuration fails when its glob matches nothing or a file cannot be read.
Why: a guard that checks zero files stays green through the regression it exists to catch.
Check: test

## Failures

### BP-TEST-024: Write failure messages that point to the fix

An assertion that can fail for a non-obvious reason carries a message that starts with a greppable word and names the operation, the input, the expected value and the file or rule to look at.
Why: the reader goes straight to the cause.
Check: review

### BP-TEST-025: Assert on the narrowest value

Compare the one field or flag that matters, not a whole document or screen.
Why: a failing comparison of a large value prints all of it and hides the difference.
Check: review

### BP-TEST-026: Read the failure before touching the test

Read the failure message and the code under test before changing a failing test.
Why: the failure usually names what broke.
Check: review

### BP-TEST-027: Keep tests as strict as written

Never weaken, skip, delete or re-record a test, or change an expectation you did not mean to change, to make a gate pass; a test the code cannot pass yet is a finding to report.
Why: a weakened test hides the regression it was written for.
Check: review

### BP-TEST-028: Skip only with a ticket

An ignored or expected-failure test names a diagnosed bug and its owner (`#[ignore = "<ticket>: <reason>"]`, `clippy::ignore_without_reason`).
Why: a reasonless skip is permanent.
Check: lint

### BP-TEST-029: Ignore tests that need optional data, never pass them

A test that needs local data the checkout may lack is ignored with a reason and fails when run without the data; it never returns early.
Why: an early return reports a pass with nothing checked.
Check: lint

### BP-TEST-030: Fix or quarantine flaky tests

A flaky test is fixed, or quarantined with an owner and a ticket; retrying until green is not a fix.
Why: retries teach everyone to ignore red.
Check: review

## User interfaces

### BP-TEST-031: Query the UI by role and name

Find elements by role and accessible name, and assert attributes, states and emitted actions; never class names or markup snapshots.
Why: tests then check what users and assistive technology get, and survive restyling.
Check: review

### BP-TEST-032: Test the view and the data path apart

Render a view with plain domain props; test the code that produces those props separately.
Why: each part fails for its own reason.
Check: review

### BP-TEST-033: Ship cases and a test with every component

Each component has a set of reproducible cases (stories) and a behaviour test; a coverage test fails when either is missing.
Why: every state is reviewable and checked.
Check: test

### BP-TEST-034: Drive time in UI tests

UI tests advance animation and timer time explicitly and check start, middle and end; nothing waits on a wall clock.
Why: waiting is slow and flaky.
Check: review

### BP-TEST-035: Compare every case visually at both extremes

Screenshot every case at the narrowest and widest supported size, with exact colours and a small pixel budget, motion, caret and theme frozen, fixed locale and timezone, after fonts and images are ready; baselines are kept per platform.
Why: a token change shows its whole blast radius.
Check: test

### BP-TEST-036: Accept visual changes one case at a time

Accept new baselines only for the cases the change was meant to alter, by case id; never re-record in bulk. A failing run lists the changed cases and the accept command.
Why: bulk re-recording approves regressions with the intended change.
Check: review

### BP-TEST-037: Opt a case out with a reason

A case excluded from visual comparison carries a flag and a comment saying why; its baseline is never just deleted.
Why: the exclusion stays visible and reversible.
Check: review

## Real dependencies

### BP-TEST-038: Gate integration tests on a dedicated variable

Tests against a real database or service run only when a dedicated test variable is set, never the production connection variable, against a version-pinned disposable instance.
Why: a developer's real data is never the target.
Check: test

### BP-TEST-039: Verify seeds before using them

After seeding a test dependency, count the records the suite relies on and fail naming what is missing.
Why: a partial seed otherwise surfaces later as an unrelated failure.
Check: test

### BP-TEST-040: Propose heavy test extras, do not add them

Database integration suites, guard suites and alternative-runtime test runs are proposed to the owner with what each would catch, and added only on request.
Why: each costs a dependency, scripts and CI time.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (tests, fixtures, fakes and suites)

## References

- [Deepening: replace, don't layer](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/DEEPENING.md)
