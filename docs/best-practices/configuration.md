---
description: How code reads environment variables and configuration, keeps the committed template in sync, and gates optional and debug behaviour.
tags: [best-practices, configuration]
---

# Configuration

Environment variables and other start-up configuration. Secrets are also covered by [[best-practices/security]].

### BP-CONF-001: Commit a template of every variable

Each app and tool commits a `.env.example` listing every variable it reads, secrets empty and safe defaults filled, each with a comment saying what it is and where to get it.
Why: a new checkout knows exactly what to set.
Check: lint

### BP-CONF-002: Add a new variable to the template in the same change

A change that reads a new variable adds its template entry in the same commit.
Why: other machines and CI lose a variable silently otherwise.
Check: lint

### BP-CONF-003: Call out every removed template entry

Removing or renaming a template entry is called out in the change and every reader is updated with it.
Why: a renamed variable keeps its old value on every machine that set it.
Check: review

### BP-CONF-004: Keep real values out of the repository

Real values live in untracked local files (`.env.local`) and the host's settings; committed env files hold nothing you would not publish.
Why: a committed secret ships with every clone ([[best-practices/security]]).
Check: lint

### BP-CONF-005: Declare every variable a task reads

Every variable a build, lint or test task reads is listed in that task's `env` in `turbo.json`; machine-local values that must not change the hash go in pass-through ([[repo/turborepo/tasks]]).
Why: strict environment mode hides undeclared variables, and an undeclared input does not invalidate the cache.
Check: lint

### BP-CONF-006: Read configuration in trusted code, near its use

Read a variable in the trusted module that uses it; expose to UI code only values explicitly marked public.
Why: a value read in UI code is shipped to every client.
Check: review

### BP-CONF-007: Validate required configuration at start-up

Parse required configuration once when the process starts and exit with a message naming each missing or malformed variable.
Why: a missing value then fails at boot, not on the first request that needs it.
Check: test

### BP-CONF-008: Derive instead of duplicating

Compute a value from an existing variable (an origin from an endpoint URL) instead of adding a second variable that can disagree.
Why: two variables for one fact eventually differ.
Check: review

### BP-CONF-009: Make optional integrations no-ops without configuration

An optional integration (telemetry, a remote cache, an external service) does nothing when its variable is unset, so local and preview builds work without production secrets.
Why: missing optional configuration must not break a build.
Check: test

### BP-CONF-010: Keep debug flags out of shared environments

Debug and verbose-logging switches default off and are set only on a developer's machine, never in CI, preview or production.
Why: debug output logs full payloads, and public build-time flags ship to every client.
Check: review

### BP-CONF-011: Resolve runtime-read files from a known root

Files read at run time by a computed path (templates, data files) are declared to the bundler or packager and resolved from a known root, never from the module's own location after bundling.
Why: bundling moves modules, and undeclared files are left out of the package.
Check: test

### BP-CONF-012: Use one variable name per concern across runtimes

A concern configured in several runtimes (the log filter, a base URL) uses the same variable name in Rust and TypeScript.
Why: one setting then configures every process.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (configuration readers and `.env.example` files)
- `turbo.json`
