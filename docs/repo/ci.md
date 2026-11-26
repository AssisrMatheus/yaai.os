---
description: Run the pull request and main checks, reproduce their selection and maintain test timings.
tags: [repo, ci, testing]
---

# Continuous integration

Every pull request and push to main starts four independent jobs on Ubuntu. New runs cancel superseded runs for the same pull request or branch. The owner requires the single check `CI passed` on main. It succeeds only when every job succeeds ([[best-practices/ci.rules]], [[invariants/ci.invariants]]).

| Job       | Local command after a frozen install                                                             |
| --------- | ------------------------------------------------------------------------------------------------ |
| Lint      | `bin/lint && bin/doc-links check && pnpm format:check && actionlint .github/workflows/check.yml` |
| Typecheck | `pnpm exec turbo run typecheck --continue && pnpm exec tsc --noEmit`                             |
| Build     | `pnpm exec turbo run build --continue`                                                           |
| Test      | `bash .github/ci/test.sh 1 2`, then repeat with shard 2                                          |

Use the Rust toolchain in `rust-toolchain.toml`, Node from `.nvmrc`, pnpm from `packageManager` and Bun 1.4.3. Install with `pnpm install --frozen-lockfile`. Lint also needs cargo-deny 0.20.2, cargo-machete 0.9.2 and actionlint 1.7.12. Lint checks the whole repository, including docs on every change. Root TypeScript checking always runs.

## Affected tests

For a PR, check out its head with full history and export `TURBO_SCM_BASE` and `TURBO_SCM_HEAD` as the base and head commit IDs. Append `--affected` to the workspace commands. The test script asks Turbo for the dry-run test graph using the same filter. A changed workspace and every workspace depending on it can contribute a test script. Mirror Rust path dependencies in workspace manifests. Knip ignores the lint runner’s mirrored dependency because Cargo consumes it. Shared root configuration and CI definitions are global inputs. Root hook tests run for hook sources, runtime hook configs, the agent picker and shared root configuration. A docs-only edit runs the docs check without workspace or hook tests. Push jobs omit affected filtering.

## Timing and caching

Bun schedules one adapter file per Cargo test target (including doctests) or Bun test file in a selected workspace. A workspace with a custom test script runs that script as one unit. The root hooks have a separate file. Bun 1.4.3 partitions contiguous path ranges, and nextest offers slice or hash partitioning. [[tooling/test-shard.tool]] assigns these files longest first to the lightest of two shards and uses the median recorded duration for an unseen file. Measurements include starting the workspace test command. The aggregate compares timing snapshot digests so different inputs to the two selectors fail the check. The committed seed in `.github/ci/timings.json` comes from a full local run. The aggregate combines `shard-*.json` from a successful main push into the 90-day `main-timings` artifact. Both shards look up the last successful main push completed before their workflow started. They use the seed when the artifact is absent or expired. Only main publishes timing updates. Increase the matrix, shard count and digest outputs together when recorded test execution dominates setup and a trial lowers the slowest job time. Split a dominating suite into finer units first. This small suite cannot establish scaling efficiency or hosted latency.

The setup action restores the pnpm store, Cargo registry, Git dependencies, target and local Turbo cache. Cargo and Turbo share a cache key per OS, architecture, job, lockfiles, Rust toolchain and commit, with a fallback under the same dependency key. Turbo runs in local-only mode. To reproduce the full run, unset both SCM variables. To benchmark without task or compilation hits, use an isolated checkout with empty target and Turbo directories.

## Initial measurements

Local Linux command wall time in seconds, one run per condition. Each job started in a separate checkout with dependencies already installed. Cold runs had empty Cargo target and Turbo caches. Warm runs repeated the same commands. These measurements exclude runner provisioning, package downloads and artifact transfers.

| Job          |  Cold | Warm |
| ------------ | ----: | ---: |
| Lint         | 16.58 | 2.07 |
| Typecheck    |  0.84 | 0.79 |
| Build        | 20.04 | 0.06 |
| Test shard 1 |  7.80 | 5.26 |
| Test shard 2 | 15.55 | 1.25 |

The initial 12-second warm budget allows a little over twice the slowest measured job. The docs-only comparison from `c287bd1` to `a3cd160` selected zero test units in a clean checkout. The slower selector took 0.061 seconds, saving 5.20 seconds on the warm test critical path. Lint still validates the edited docs.

## Add a job

Declare the job beside the existing jobs, give it read-only permissions, pin each action SHA with its release in a comment and add its result to the aggregate. Check the workflow with actionlint and measure the cold and warm commands before raising the budget.

## Covers

- `.github/`
- `Cargo.toml`
- `package.json`
- `tsconfig.json`
- `turbo.json`
- `tooling/lint/package.json`

## References

- https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
- https://turborepo.dev/docs/reference/run#--affected
- https://github.com/oven-sh/bun/blob/bun-v1.4.3/src/runtime/cli/test/Timings.rs
- https://nexte.st/docs/ci-features/partitioning/
