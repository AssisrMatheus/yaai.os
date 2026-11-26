---
description: Where a file goes, how modules and packages are split and exported, and how generated files are kept in step with their sources.
tags: [best-practices, structure]
---

# Code organisation

Workspace kinds, names and task wiring are set by [[repo/structure]] and [[repo/turborepo/tasks]]; this page covers
how code is laid out inside them.

## Files and modules

### BP-ORG-001: Place a file by its runtime

Put trusted-side code (I/O, secrets, services), UI-only code and runtime-neutral code in separate modules; runtime-neutral modules do no I/O, hold no secrets and touch no platform globals.
Why: the location tells a reviewer what an import costs and where it may run.
Check: review

### BP-ORG-002: Keep one runtime per module

A module is either trusted-side, UI-side or neutral; code shared by both sides is neutral.
Why: a mixed module drags server code into a client bundle or the reverse.
Check: review

### BP-ORG-003: Name files after what they do

Name a module after its job (`image_sizes.rs`, `scanner_probe.ts`); never a `utils` or `helpers` grab-bag.
Why: a grab-bag grows without owner and hides duplicates.
Check: review

### BP-ORG-004: Colocate what changes together

Keep a module's tests, fixtures, schema, styles and local parts beside it; give only cross-cutting suites their own folder.
Why: one change then touches one folder.
Check: review

### BP-ORG-005: Give each component or module its own folder once it has parts

A unit with private parts gets a folder holding its entry file and those parts; the parts stay private to it.
Why: the folder is the unit of review and deletion.
Check: review

### BP-ORG-006: Import granular modules, never barrels

Import from the module that defines an item; write no file that only re-exports everything (oxlint `oxc/no-barrel-file`).
Why: barrels defeat tree-shaking and drag unrelated code, including trusted-side modules, into bundles.
Check: lint

### BP-ORG-007: Expose package modules through granular exports

A package exposes each module through a wildcard export (`"./*": "./src/*.ts"`) or explicit entries, and consumers import from those entries, never a package root or internal path.
Why: new modules need no manifest edit, and internals stay private.
Check: lint

### BP-ORG-008: Declare side effects accurately

A package declares `"sideEffects": false` unless a module has import-time effects, which are then listed.
Why: bundlers drop unused modules only when told it is safe.
Check: review

## Packages

### BP-ORG-009: Create a shared package for two consumers, not one

Move code into `packages/` when two workspaces use it today; until then it stays in the workspace that uses it.
Why: extracting early freezes an interface before a second caller tests it.
Check: review

### BP-ORG-010: Point dependencies from apps to packages

Apps depend on packages; a package never imports an app and an app never imports another app (INV-STRUCTURE-001).
Why: apps talk through network or IPC contracts, so each builds and deploys alone.
Check: lint

### BP-ORG-011: Declare cross-workspace file reads as task inputs

A task that reads another workspace's files by path lists them in its `inputs` in `turbo.json`, and the exception is named in the governing doc.
Why: otherwise the task replays from cache after the files it reads changed.
Check: review

### BP-ORG-012: Behaviour moves to a package, presentation stays

When a second consumer needs a component's behaviour (a gesture, a timing loop), move the behaviour into a package and keep markup, names and styles in each app.
Why: shared behaviour gets one fix; presentation stays free to differ.
Check: review

## Generated files

### BP-ORG-013: Change generated files through their source

Change the source and regenerate; a generated file carries a header saying so and how to regenerate it.
Why: the next generation overwrites hand edits.
Check: review

### BP-ORG-014: Make generation a cached task

Generation runs as its own task whose outputs are excluded from its inputs, and build, lint and typecheck depend on it.
Why: a clean checkout generates before it checks.
Check: review

### BP-ORG-015: Check committed generated files against their source

A committed generated file has a check that regenerates it into a temporary location and fails on any difference.
Why: drift between source and output is caught in the change that caused it.
Check: test

### BP-ORG-016: Commit generated output with its source change

A change to a schema, query or other generator input commits the regenerated output in the same change.
Why: the repository is never in a state where the two disagree.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (module layout, package manifests and generated files)
