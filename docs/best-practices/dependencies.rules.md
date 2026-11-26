---
description: When to add a dependency, how it is declared, checked, pinned and upgraded, and how its documentation is read.
tags: [best-practices, dependencies]
---

# Dependencies

Where a dependency is declared is set by [[repo/structure]]; this page covers whether and how. Rust first, then
TypeScript.

## Choosing

### BP-DEPS-001: Look in the standard library and the workspace first

Check `std`, the existing workspace crates and packages, and the dependencies already in use before adding a new one.
Why: every dependency is code to audit, build and upgrade.
Check: review

### BP-DEPS-002: Justify each dependency

A new dependency states its purpose in the commit, and has a declared licence, a trusted source and a compatible minimum Rust or runtime version.
Why: unjustified dependencies accumulate.
Check: review

### BP-DEPS-003: Count build cost for build-time dependencies

Treat procedural macros and build scripts as dependencies whose build-time cost is reviewed.
Why: they slow every build and run code at compile time.
Check: review

### BP-DEPS-004: Prefer the existing primitive for a concern

Before adding a library for a concern (a widget, a positioning engine, a carousel), check that the layer already chosen does not cover it, and justify a second library for the same concern.
Why: two libraries for one concern mean two behaviours on one screen.
Check: review

### BP-DEPS-005: Rebuild catalogue snippets, never paste them

Code from a component catalogue or snippet site is read for structure and rebuilt on the repository's own primitives and tokens.
Why: pasted code brings its own dependencies and conventions.
Check: review

## Declaring

### BP-DEPS-006: Declare a dependency where it is imported

Add a dependency to the crate or workspace that imports it, never to the root for one workspace's code (`pnpm add <pkg> --filter @yaai/<dir>`, the member crate's `Cargo.toml`).
Why: an undeclared dependency builds locally and breaks when the workspace is built alone.
Check: review

### BP-DEPS-007: Remove unused dependencies

No declared dependency goes unused (`cargo machete`, an unused-dependency check for packages).
Why: unused dependencies still cost build time and audits.
Check: lint

### BP-DEPS-008: Keep development dependencies out of release code

Test and build helpers are dev-dependencies and never reach release artifacts.
Why: release binaries ship only what they run.
Check: lint

### BP-DEPS-009: Put optional integrations behind additive features

Gate an optional integration behind a Cargo feature (`metadata = ["dep:metadata-reader"]`); enabling one feature never breaks another, because features unify across callers.
Why: users of one feature must not pay for or be broken by another.
Check: review

### BP-DEPS-010: Disable unused default features

Turn off a dependency's default features and enable only those the code uses.
Why: defaults pull in code, build time and attack surface.
Check: review

### BP-DEPS-011: Test the supported feature combinations

Build and test default features, no default features and every supported combination on each supported target.
Why: an untested combination breaks the first user who needs it.
Check: test

## Checking and pinning

### BP-DEPS-012: Check advisories, licences and sources

`cargo deny check` passes for advisories, bans, licences and sources, including duplicate versions and wildcard requirements.
Why: known vulnerabilities and unknown sources are caught before merge.
Check: lint

### BP-DEPS-013: Commit lockfiles and install frozen

`Cargo.lock` and `pnpm-lock.yaml` are committed; CI and hooks install with `--locked` and `--frozen-lockfile`.
Why: every machine builds the same dependency graph.
Check: lint

### BP-DEPS-014: Review lockfile changes with the manifest

A lockfile change is reviewed together with the manifest change that caused it; an unexplained lockfile change is questioned.
Why: unexpected upgrades hide in lockfile churn.
Check: review

### BP-DEPS-015: Upgrade a package family together

Packages released as one family stay on the same version and are upgraded together.
Why: mismatched family versions fail at run time with errors that do not name the mismatch.
Check: review

### BP-DEPS-016: Install released versions only

Install a dependency from a published release; never from a branch, a commit or a tarball. Peer dependencies use exactly the version the release pins.
Why: unreleased code changes under you.
Check: review

### BP-DEPS-017: Stop when an install is refused

When a registry, an access token or a release-age policy refuses an install, stop and report which to the owner; never loosen the policy or swap the version.
Why: the policy is a decision, not an obstacle.
Check: review

### BP-DEPS-018: Use pnpm and bun only as decided

JavaScript packages are installed with pnpm only; bun runs scripts and tests ([[best-practices/typescript.rules]]).
Why: one package manager, one lockfile.
Check: lint

### BP-DEPS-019: Review install-script permissions

A package whose install script must run is allowed explicitly in the workspace configuration, after review.
Why: install scripts run arbitrary code on every machine.
Check: review

### BP-DEPS-020: Pin tool versions centrally

Pin the Rust toolchain in `rust-toolchain.toml`, the package manager in `packageManager`, and tool versions such as `bun` exactly, once, at the root.
Why: every machine and CI run uses the same tools.
Check: review

### BP-DEPS-021: Keep vendored sources read-only

Copies under `vendor/` and vendored third-party guides are read-only; changes go upstream or into the repository's own code.
Why: an edited copy diverges silently and is lost on the next sync.
Check: lint

### BP-DEPS-022: Keep vendor template files as shipped

Files a dependency generates or copies into the project as a template stay untouched; customise through its configuration.
Why: upgrades replace them and drop local edits.
Check: review

## Reading documentation

### BP-DEPS-023: Read the installed version's docs before using an API

Before writing an API from memory, read the documentation matching the installed version, in this order of trust: docs shipped with the package or generated locally (`cargo doc`), the docs page as markdown, the site's `llms.txt`, then a skill.
Why: memory is often a version behind.
Check: review

## Covers

- `Cargo.toml`
- `Cargo.lock`
- `package.json`
- `pnpm-lock.yaml`
- `pnpm-workspace.yaml`
- `rust-toolchain.toml`
- `apps/`, `packages/`, `tooling/` (manifests)
