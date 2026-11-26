---
description: How TypeScript code applies the Rust rules, and the rules that exist only in TypeScript (types, modules, promises, oxlint, oxfmt, bun).
tags: [best-practices, typescript]
---

# TypeScript

TypeScript is used only where Rust does not fit (ADR 0002, INV-STRUCTURE-002). Every Rust rule listed below applies,
with the TypeScript form in parentheses; the rules after the list exist only in TypeScript. Cross-cutting pages
(logging, testing, security, UI) apply to TypeScript as written; package entry points and barrel files are
[[best-practices/code-organization.rules]] (BP-ORG-006 to BP-ORG-008).

## Shared with Rust

- BP-TS-001: Same as [[best-practices/rust.rules#BP-RUST-001: Use glossary terms in code]]
- BP-TS-002: Same as [[best-practices/rust.rules#BP-RUST-002: Give each identity its own newtype]] (a branded type, `type TitleId = string & { readonly brand: 'TitleId' }`)
- BP-TS-003: Same as [[best-practices/rust.rules#BP-RUST-003: Model exclusive states as an enum]] (a discriminated union on a literal `kind` field)
- BP-TS-004: Same as [[best-practices/rust.rules#BP-RUST-004: Model absence with Option]] (an optional property or `undefined`; never `''`, `-1` or a mix of `null` and `undefined`)
- BP-TS-005: Same as [[best-practices/rust.rules#BP-RUST-005: Match owned enums exhaustively]] (`satisfies Record<Union, ...>` or a `never` check in the `default` branch)
- BP-TS-006: Same as [[best-practices/rust.rules#BP-RUST-006: Parse external values into an enum before branching]] (narrow with a schema parser or `Object.hasOwn` before indexing; BP-TS-063)
- BP-TS-007: Same as [[best-practices/rust.rules#BP-RUST-007: Keep content in data]]
- BP-TS-008: Same as [[best-practices/rust.rules#BP-RUST-008: Name thresholds and limits]]
- BP-TS-009: Same as [[best-practices/rust.rules#BP-RUST-009: Carry units in types]] (branded number types or a date-time value, never a bare millisecond count without a named type)
- BP-TS-010: Same as [[best-practices/rust.rules#BP-RUST-011: Prefer expressions and inferred locals]] (infer locals and return types; BP-TS-059)
- BP-TS-011: Same as [[best-practices/rust.rules#BP-RUST-012: Use the language before hand-rolling]] (`??=`, `?.`, `satisfies`, `using`, iterator helpers)
- BP-TS-012: Same as [[best-practices/rust.rules#BP-RUST-026: Document every item]] (a one-line intent JSDoc on every function and method)
- BP-TS-013: Same as [[best-practices/rust.rules#BP-RUST-028: State intent, then contract]] (in JSDoc)
- BP-TS-014: Same as [[best-practices/rust.rules#BP-RUST-031: Cite the governing doc above the item]]
- BP-TS-015: Same as [[best-practices/rust.rules#BP-RUST-032: Comment the why]]
- BP-TS-016: Same as [[best-practices/rust.rules#BP-RUST-033: Give every TODO an owner]]
- BP-TS-017: Same as [[best-practices/rust.rules#BP-RUST-035: Mark comments with Better Comments prefixes]]
- BP-TS-018: Same as [[best-practices/rust.rules#BP-RUST-036: Delete dead code]]
- BP-TS-019: Same as [[best-practices/rust.rules#BP-RUST-039: Fix the code, not the lint]] (`// oxlint-disable-next-line <rule> -- <reason>`; BP-TS-081)
- BP-TS-020: Same as [[best-practices/rust.rules#BP-RUST-040: Change lint policy only by owner decision]]
- BP-TS-021: Same as [[best-practices/rust.rules#BP-RUST-043: Run the same gates everywhere]]
- BP-TS-022: Same as [[best-practices/rust-errors.rules#BP-RUST-044: Return typed errors, never panic]] (return expected failures as values, never `throw` them; BP-TS-074)
- BP-TS-023: Same as [[best-practices/rust-errors.rules#BP-RUST-045: Define the error beside the operation]] (an `Error` subclass in the file of the operation; BP-TS-076)
- BP-TS-024: Same as [[best-practices/rust-errors.rules#BP-RUST-046: One variant per caller decision]]
- BP-TS-025: Same as [[best-practices/rust-errors.rules#BP-RUST-047: Keep the cause chain]] (`new XError(message, { cause })`)
- BP-TS-026: Same as [[best-practices/rust-errors.rules#BP-RUST-048: Propagate unchanged with ?]]
- BP-TS-027: Same as [[best-practices/rust-errors.rules#BP-RUST-049: Separate absence from failure]] (`undefined` for a genuine miss, an error value for a failed read)
- BP-TS-028: Same as [[best-practices/rust-errors.rules#BP-RUST-050: Separate incomplete from unsupported input]]
- BP-TS-029: Same as [[best-practices/rust-errors.rules#BP-RUST-052: Compare errors by variant or code]]
- BP-TS-030: Same as [[best-practices/rust-errors.rules#BP-RUST-053: Map errors once, at the outer boundary]]
- BP-TS-031: Same as [[best-practices/rust-errors.rules#BP-RUST-054: Convert foreign failures in one place]] (catch as `unknown`, narrow, return the domain error)
- BP-TS-032: Same as [[best-practices/rust-errors.rules#BP-RUST-055: Report an unexpected failure once]]
- BP-TS-033: Same as [[best-practices/rust-errors.rules#BP-RUST-056: Give the caller a safe remedy]]
- BP-TS-034: Same as [[best-practices/rust-errors.rules#BP-RUST-058: Give wire errors a stable code]] (branch on `name` or `code`, never `instanceof`, after a value crossed a boundary)
- BP-TS-035: Same as [[best-practices/rust-errors.rules#BP-RUST-059: Parse into validated types at entry]]
- BP-TS-036: Same as [[best-practices/rust-errors.rules#BP-RUST-062: Check the whole format]]
- BP-TS-037: Same as [[best-practices/rust-errors.rules#BP-RUST-063: Drop unknown values and log them]]
- BP-TS-038: Same as [[best-practices/rust-ownership.rules#BP-RUST-072: Keep one owner of mutable state]]
- BP-TS-039: Same as [[best-practices/rust-ownership.rules#BP-RUST-074: Let the caller supply storage]]
- BP-TS-040: Same as [[best-practices/rust-ownership.rules#BP-RUST-075: Pass time, randomness and I/O in]] (no `Date.now()`, `performance.now()` or `Math.random()` inside the computation)
- BP-TS-041: Same as [[best-practices/rust-ownership.rules#BP-RUST-076: Order iteration deterministically]] (sort before output; object key order is not a contract)
- BP-TS-042: Same as [[best-practices/rust-ownership.rules#BP-RUST-077: Name the hot paths]]
- BP-TS-043: Same as [[best-practices/rust-ownership.rules#BP-RUST-078: Allocate nothing in a hot path]] (reuse arrays, typed arrays and objects; no spreads or closures per frame)
- BP-TS-044: Same as [[best-practices/rust-ownership.rules#BP-RUST-080: Count allocations on each hot path]]
- BP-TS-045: Same as [[best-practices/rust-async.rules#BP-RUST-084: Await dependent work in order, start independent work together]] (`Promise.all` for independent work)
- BP-TS-046: Same as [[best-practices/rust-async.rules#BP-RUST-085: Bound concurrency with a documented limit]]
- BP-TS-047: Same as [[best-practices/rust-async.rules#BP-RUST-088: Give every task an owner]] (an `AbortController` per owner)
- BP-TS-048: Same as [[best-practices/rust-async.rules#BP-RUST-089: Put a deadline on every external call]] (`AbortSignal.timeout`)
- BP-TS-049: Same as [[best-practices/rust-async.rules#BP-RUST-090: Retry only transient, idempotent work]]
- BP-TS-050: Same as [[best-practices/rust-async.rules#BP-RUST-092: Discard stale results]]
- BP-TS-051: Same as [[best-practices/rust-async.rules#BP-RUST-097: Merge results in input order]]
- BP-TS-052: Same as [[best-practices/rust-async.rules#BP-RUST-098: Pick failures deterministically]]
- BP-TS-053: Same as [[best-practices/rust-async.rules#BP-RUST-101: Keep mutable state out of globals]] (no mutable `let` or object at module scope)
- BP-TS-054: Same as [[best-practices/rust-async.rules#BP-RUST-102: Tie subscriptions to their owner]]
- BP-TS-086: Same as [[best-practices/rust-ownership.rules#BP-RUST-121: Model a non-trivial state transition as a pure function]] (a reducer `(state, action) => nextState`, exhaustive over the action union, effects at the calling boundary)

## Tooling

### BP-TS-055: Use TypeScript only where Rust does not fit

Write new code in Rust; choose TypeScript for a browser surface or an integration Rust cannot serve, and say why in the governing doc.
Why: one language for logic keeps rules, tests and tools in one place (ADR 0002).
Check: review

### BP-TS-056: Install with pnpm, run with bun

Manage packages with `pnpm` only and run scripts and tests with `bun`; no other package manager installs anything.
Why: one lockfile and one resolver for the workspace ([[best-practices/dependencies.rules]]).
Check: lint

### BP-TS-057: Format with oxfmt, lint with oxlint at zero warnings

`oxfmt --check` and `oxlint --deny-warnings` pass on every TypeScript file.
Why: warnings that never fail are never fixed.
Check: lint

### BP-TS-058: Typecheck strictly

`tsc --noEmit` passes with `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes` and `erasableSyntaxOnly` enabled.
Why: the compiler then checks what the Rust compiler checks by default.
Check: lint

## Types

### BP-TS-059: Infer return types and locals

Annotate no return type and no local whose initialiser states its type; let the compiler infer error unions.
Why: an explicit return type hides a new failure branch as the implementation grows.
Check: review

### BP-TS-060: Derive deferred annotations

A binding assigned later (in a `try`, loop or branch) is annotated with a type derived from its source (`ReturnType`, `Awaited`, `Parameters`, indexed access), not a restated shape.
Why: the derived type follows the source when it changes.
Check: review

### BP-TS-061: Name a type only for a shared contract

Declare a named type only when another module references the shape; check each implementation with `satisfies`.
Why: a type written beside its only implementation is the same shape declared twice.
Check: review

### BP-TS-062: Check shapes with satisfies

Use `satisfies` to check an object against a contract; use `as` only for a narrowing the compiler cannot prove, with a comment saying why (`as const` is fine).
Why: `satisfies` reports errors and keeps literal types; `as` silences them.
Check: lint

### BP-TS-063: Narrow untrusted data, never cast it

Treat network, file, storage and message data as `unknown` and narrow it with a schema parser; check map keys with `Object.hasOwn`, not `in`.
Why: a cast turns a hoped-for shape into one the compiler trusts; `in` also matches inherited keys.
Check: lint

### BP-TS-064: Use unknown, never any

Write no `any`, including `catch (error: any)` and generic defaults; variadic helpers take `unknown[]` and `catch` narrows with `instanceof Error` (`typescript/no-explicit-any`).
Why: `any` switches the type checker off for everything it touches.
Check: lint

### BP-TS-065: Declare shapes with type

Use `type` for shapes; use `interface` only for declaration merging or extending a third-party interface (`typescript/consistent-type-definitions`).
Why: one form for one purpose.
Check: lint

### BP-TS-087: Compose contracts with intersections

When a value must honour several contracts, name the composition once (`type Controller = Behaviour & Disposable`) and check the implementation with `satisfies` against it.
Why: each contract stays defined once, and the implementation keeps its full inferred type.
Check: review

### BP-TS-066: Use literal unions, never enum

Model closed sets as a literal union or an `as const` object; write no `enum` (rejected by `erasableSyntaxOnly`).
Why: enums emit runtime code and are nominal, so plain strings do not assign to them.
Check: lint

### BP-TS-067: Branch three or more cases with a checked map

For three or more cases on one value, use `as const satisfies Record<Union, Handler>` instead of a `switch` or `if` chain, and decide an explicit fallback for a value that failed narrowing.
Why: a new union member is a compile error until handled.
Check: review

```ts
const STATUS_KEY = {
  idle: 'status.idle',
  playing: 'status.playing',
  paused: 'status.paused'
} as const satisfies Record<PlaybackStatus, MessageKey>;
```

### BP-TS-068: Mark boundary data readonly

Data that crosses a module boundary (a public type, a message, a snapshot) is `readonly`; updates use spreads.
Why: the compiler enforces "do not mutate what you received".
Check: review

### BP-TS-069: Reuse platform element types

Type element props with the platform's own types (`ComponentProps<'button'>`) instead of hand-written attribute lists.
Why: every native attribute stays available and correctly typed.
Check: review

### BP-TS-070: Return new values instead of mutating arguments

Return the next value instead of changing the one you received; mutate in place only on a measured hot path, with the invariant in the doc comment.
Why: callers keep references to what they passed.
Check: lint

## Functions and modules

### BP-TS-071: Prefer functions and create factories

Compose with functions and `create*` factories that keep private state in a closure and return a plain object; use a class only for an `Error` subclass or a third-party base class.
Why: no `this` to lose, real privacy, and unused factories tree-shake.
Check: review

### BP-TS-072: Use named exports and inline type imports

Export by name and import types inline and sorted (`import { type Catalog } from './catalog'`, `typescript/consistent-type-imports`).
Why: imports stay explicit and type-only imports erase cleanly.
Check: lint

### BP-TS-088: Run nothing at import time

Module scope holds declarations and constants only: no I/O, timers, listeners, registrations or instance creation when the module is imported; work starts when a caller calls a function or factory.
Why: importing a module then has no effect, so unused code tree-shakes and tests import it safely.
Check: review

### BP-TS-073: Default with ??, not ||

Use `??` and `??=` for defaults; use `||` only for boolean logic (`typescript/prefer-nullish-coalescing`).
Why: `||` replaces valid `0`, `''` and `false`.
Check: lint

## Errors

### BP-TS-074: Return results, not exceptions

Success is `{ ok: true as const, ...data }`; an expected failure is a returned instance of a domain `Error` subclass. Never return `{ ok: false }` or put `ok` on an error.
Why: one presence check tells success from failure everywhere.
Check: review

### BP-TS-075: Narrow results by presence of ok

Check `if (!('ok' in result)) return result;` and propagate the error unchanged; use `instanceof` only after this check to tell failure variants apart, or in a `catch` after narrowing `unknown`.
Why: the check is correct by construction because only success has the key.
Check: review

### BP-TS-076: Give every error class a literal name

Each `Error` subclass sets `readonly name = 'XError'`, plus a literal `code` when callers need a stable reason and a literal `statusCode` when an HTTP layer maps it.
Why: name and code are the checks that survive serialisation.
Check: review

## Promises

### BP-TS-077: Return values and promises directly

Return synchronous values and existing promises directly: no `Promise.resolve` or `Promise.reject` to make a contract asynchronous, and no `async` on a function that only forwards a promise (`typescript/require-await`).
Why: extra wrapping hides whether work is asynchronous.
Check: lint

### BP-TS-078: Sequence with async and await

Use `async`/`await` with `try`/`catch`/`finally` instead of `.then()`/`.catch()` chains.
Why: control flow reads top to bottom.
Check: lint

### BP-TS-079: Keep the Promise global narrow

Use `Promise.all`, `allSettled`, `any` and `race` to coordinate, and `new Promise` only in a small adapter for one callback or event API that settles on every path; any other runtime use of `Promise` carries a comment with its interop reason.
Why: other uses usually reimplement `async` badly.
Check: review

### BP-TS-080: Handle every promise

Await, return or explicitly handle every promise; no floating promises (`typescript/no-floating-promises`).
Why: an unhandled rejection is a lost failure.
Check: lint

## Suppressions, tests and exit

### BP-TS-081: Suppress one line, with the rule and a reason

When a suppression is right, write `// oxlint-disable-next-line <rule> -- <reason>`; never a file-wide disable outside generated code.
Why: the reason lets a reviewer accept it, and the scope stays one line.
Check: lint

### BP-TS-082: Put bun tests beside their source

Tests import from `bun:test` and sit beside the module as `<name>.test.ts`; only suites spanning many modules get a folder.
Why: the test moves and is reviewed with its code.
Check: review

### BP-TS-083: Skip visibly when data is missing

A test that needs optional local data uses `test.skipIf(...)` so it reports as skipped; it never returns early inside the body.
Why: an early return reports a pass with nothing checked.
Check: review

### BP-TS-084: Set exitCode, do not call process.exit

A CLI entry sets `process.exitCode` and returns instead of calling `process.exit()` (`unicorn/no-process-exit`).
Why: pending writes flush before the process ends.
Check: lint

### BP-TS-085: Treat every index as possibly undefined

Handle the `undefined` that `noUncheckedIndexedAccess` adds to indexed reads; never silence it with `!` (`typescript/no-non-null-assertion`).
Why: this is the TypeScript form of checked accessors.
Check: lint

## Covers

- `apps/`, `packages/`, `tooling/` (TypeScript sources, `package.json` and `tsconfig.json` files)
- `.oxlintrc.json`
- `.oxfmtrc.json`

## References

- [Codebase design](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/SKILL.md)
