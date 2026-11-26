---
description: What git hooks and CI run, how a single required check is built, what a clean checkout must prove, and how a check result is read.
tags: [best-practices, ci, testing]
---

# Hooks and CI

Where tests and gates run. The tiers themselves are in [[best-practices/testing.rules]]; commit format and hooks wiring are
in [[repo/commits]] and [[repo/hooks]].

## Hooks

### BP-CI-001: Run only the fast tier before commit

The pre-commit hook runs typecheck, unit tests and staged-file lint only; when it gets slow, move the slow test to a later tier instead of tolerating it.
Why: a slow hook gets bypassed.
Check: review

### BP-CI-002: Land changes through the hooks

Never commit with `--no-verify` or otherwise skip hooks to land a change whose checks fail.
Why: the hook is the first gate everyone shares.
Check: lint

### BP-CI-003: Unset git's local variables in hooks

A hook that runs nested tools which call git unsets the repository-local variables git exports to hooks (`unset $(git rev-parse --local-env-vars)`).
Why: in a linked worktree those variables point nested tools at the wrong repository.
Check: review

## The required check

### BP-CI-004: Gate merges on one aggregate check

Suites run as separate jobs; one final job that always runs fails when any of them failed or was cancelled, and only that job is required.
Why: a skipped required job counts as passed, and one stable name survives adding or splitting jobs.
Check: review

### BP-CI-005: Filter paths inside the run, not on the trigger

Never put a path filter on the trigger of a workflow that holds a required check; skip work inside the run, excluding only files no suite reads.
Why: a required check whose workflow never starts blocks the merge forever.
Check: review

### BP-CI-006: Keep CI self-sufficient

CI needs no external service, content source or third party; fakes start inside the suites, and a real service container is added only for an opted-in integration suite.
Why: CI results depend only on the commit.
Check: review

### BP-CI-007: Keep CI check-only

The check workflow never deploys, migrates a real database or reads a production secret; variables the build needs get non-secret placeholders.
Why: a check must be safe to run on any branch.
Check: review

### BP-CI-008: Pin renderers to the test library version

A rendering or browser image used by visual tests is pinned to the exact version of the test library, and a step fails when they disagree.
Why: a dependency bump must not silently change the renderer.
Check: test

### BP-CI-009: Pin images and actions by digest

Pin service images by digest and third-party actions by commit SHA with the version in a comment; grant the workflow read-only permissions by default.
Why: tags move; SHAs do not.
Check: review

### BP-CI-010: Upload failure evidence

On failure, upload the reports, diffs, traces and logs a person needs to decide, with a retention period.
Why: a failure nobody can inspect gets re-run instead of fixed.
Check: review

### BP-CI-011: Keep CI fast

Run suites as parallel jobs, cancel a superseded run, restore the task cache keyed per job and commit, and run tasks with `--continue` so one failure does not hide others.
Why: slow CI is skipped or batched.
Check: review

### BP-CI-012: Keep runner-only limits in the workflow

Limits only the CI runner needs (memory flags, worker counts) are set on the workflow step, not in a committed script.
Why: they would also apply to developer machines.
Check: review

## Clean-checkout gates

### BP-CI-013: Generate before checking

CI runs every generation step (code generation, type generation) explicitly before lint and typecheck.
Why: a developer machine has generated files a clean checkout lacks.
Check: review

### BP-CI-014: Check committed generated files for drift

After regenerating, CI fails when a committed generated file differs (`git diff --exit-code -- <path>`).
Why: a stale generated file disagrees with its source silently.
Check: test

### BP-CI-015: Lint in check mode

CI runs linters and formatters in check mode, or fails when fix mode changed any file.
Why: an auto-fix on the runner hides a violation the branch still carries.
Check: lint

### BP-CI-016: Leave the tree as found

After generation and build, no tracked file differs by content and no untracked file appeared.
Why: an undeclared generation step shows up as a dirty tree.
Check: test

### BP-CI-017: Keep builds off third parties

A build that would reach a third party is pointed at a loopback fixture in CI, and the job asserts the fixture served requests.
Why: a green build with zero requests went somewhere else, or nowhere.
Check: test

### BP-CI-018: Install from frozen lockfiles

CI installs with `pnpm install --frozen-lockfile` and builds Rust with `--locked`.
Why: CI tests the dependencies the commit declares.
Check: lint

## Reading results

### BP-CI-019: Treat a check as evidence for one commit

Before calling a change green, compare the run's head SHA with the branch head; an absent check did not run and is not a pass.
Why: a result for another commit proves nothing about this one.
Check: review

### BP-CI-020: Investigate a failure instead of re-running it

Do not re-run a failed job hoping for a different result; an unexplained failure is a finding to report.
Why: re-runs hide flaky tests and real races.
Check: review

### BP-CI-021: Leave repository settings to the owner

Making a check required, changing branch rules or adding a remote-cache credential is the owner's decision; an agent reports what is needed.
Why: these are account and policy settings.
Check: review

## Covers

- `.husky/`
- `lint-staged.config.mjs`
