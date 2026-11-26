---
name: yaai-best-practices
description: Coding rules for every app, package and tool, Rust first. Use when writing or reviewing code, choosing a pattern, adding a dependency, test, log line, cache, form or screen, or mapping a rule to a lint.
tags: [best-practices, index]
---

# Coding best practices

The rules code in `apps/`, `packages/` and `tooling/` follows. Rust is the base language; other languages map to it.
No app architecture or UI framework is chosen yet, so these rules name principles, not libraries.

Each rule is one checkable statement under a stable id, `BP-<AREA>-NNN`, in its heading: the rule, a `Why:` line, and
a `Check:` line. `Check: lint` means a compiler or linter rule can enforce it, `Check: test` means a test enforces
it, `Check: review` means it needs judgement. Cite a rule by its id in code comments, lint messages and reviews.
Ids are never renumbered or reused; a new rule takes the next free number of its prefix, wherever it is placed.

## Which file to read

| You want to | Read |
| --- | --- |
| Name, type, document or lint Rust code | [[best-practices/rust.rules]] |
| Return, propagate or map errors; parse untrusted input | [[best-practices/rust-errors.rules]] |
| Resolve a borrow conflict, keep computation pure, optimise a hot path | [[best-practices/rust-ownership.rules]] |
| Write async code, tasks, threads or shared state | [[best-practices/rust-async.rules]] |
| Write `unsafe` code or an FFI boundary | [[best-practices/rust-unsafe.rules]] |
| Write TypeScript | [[best-practices/typescript.rules]] |
| Design a module's interface, a seam or an adapter | [[best-practices/module-design.rules]] |
| Read or write an external system, validate input, shape a service | [[best-practices/data-boundaries.rules]] |
| Cache a read: keys, layer, lifetimes, failures | [[best-practices/caching.rules]] |
| Invalidate a cache: tags, flushes, change notifications, client query caches | [[best-practices/cache-invalidation.rules]] |
| Read an environment variable or configuration | [[best-practices/configuration.rules]] |
| Handle secrets, TLS, escaping, uploads or access control | [[best-practices/security.rules]] |
| Log, report or filter a failure | [[best-practices/logging.rules]] |
| Build a CLI: output, exit codes, speed | [[best-practices/tooling-cli.rules]] |
| Write or fix a test | [[best-practices/testing.rules]] |
| Touch a git hook, CI workflow or clean-checkout gate | [[best-practices/ci.rules]] |
| Measure or improve speed, size or load | [[best-practices/performance.rules]] |
| Add, upgrade or configure a dependency | [[best-practices/dependencies.rules]] |
| Decide where a file, package or generated file lives | [[best-practices/code-organization.rules]] |
| Build a screen or component | [[best-practices/ui.rules]] |
| Hold UI state, run an effect, load async data in a screen | [[best-practices/ui-state.rules]] |
| Add links, prefetching, click feedback or screen transitions | [[best-practices/navigation.rules]] |
| Build a form | [[best-practices/forms.rules]] |
| Make anything a person sees or operates accessible | [[best-practices/accessibility.rules]] |
| Write a user-visible string, format a number or date, handle locales | [[best-practices/i18n.rules]] |
| Style a component: tokens, themes, variants, typography | [[best-practices/design-tokens.rules]] |
| Animate or transition anything | [[best-practices/motion.rules]] |
| Render an image, icon, font or video embed | [[best-practices/assets.rules]] |
