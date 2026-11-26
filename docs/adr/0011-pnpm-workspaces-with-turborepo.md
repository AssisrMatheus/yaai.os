---
description: Decide to manage the monorepo with pnpm workspaces and Turborepo task orchestration.
tags: [adr, structure, tooling]
status: accepted
---

# pnpm workspaces with Turborepo

On 2026-11-26 the owner decided to organize the monorepo with `pnpm` workspaces and `Turborepo` for task orchestration and caching ([[repo/structure]], [[repo/turborepo/SKILL]]). Strict workspace isolation in `pnpm` prevents phantom dependencies, while Turborepo caches deterministic `lint`, `typecheck`, `test`, and `build` runs across workspaces so agent verification stays fast.

## Considered options

- **Plain npm/yarn/bun workspaces without a task cache**: rejected because running checks across growing workspaces without a content-hashed task graph slows edit and stop verification loops.

## Consequences

- Root `package.json` delegates workspace tasks to `turbo run`; agent hooks enforce `pnpm` over other JavaScript package managers.
