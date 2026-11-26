---
description: How to shape a module's interface, where to put a seam, when an adapter is justified, and how domain terms and decisions are recorded.
tags: [best-practices, design]
---

# Module design

The vocabulary and rules for designing deep modules, in any language. Examples are Rust.

## Vocabulary

### BP-MOD-001: Use the design vocabulary

Say **module** (anything with an interface and an implementation), **interface** (everything a caller must know: signature, invariants, ordering, errors, configuration, cost), **implementation**, **depth**, **seam**, **adapter**, **leverage** and **locality**; avoid "component", "service" or "boundary" as synonyms for them.
Why: one word per idea keeps design discussions and docs unambiguous.
Check: review

## Interfaces

### BP-MOD-002: Design deep modules

Put a lot of behaviour behind a small interface: few entry points, simple parameters, complexity hidden inside.
Why: depth gives callers leverage and maintainers locality.
Check: review

### BP-MOD-003: Document the whole interface

The module's rustdoc states its invariants, ordering constraints, error modes, required configuration and performance cost, not only the signature.
Why: a caller must know these to use it correctly.
Check: review

### BP-MOD-004: Apply the deletion test

Imagine deleting the module: if its complexity vanishes it was a pass-through and goes; if complexity reappears across callers it earns its place.
Why: shallow modules add interface without adding behaviour.
Check: review

### BP-MOD-005: Name the interface after the caller's need

Name operations after what the caller accomplishes (`Catalog::find_title`, `Library::list_recent`), never after a provider's API or as a generic `query(collection, filter)`.
Why: a generic port ends up expressing the one provider it was written against.
Check: review

### BP-MOD-006: Test through the interface

Callers and tests cross the same seam; a test that needs to reach past the interface means the module has the wrong shape.
Why: the interface is the test surface ([[best-practices/testing]]).
Check: review

### BP-MOD-007: Start private, widen for a real caller

Make items private, widen to `pub(super)` or `pub(crate)` for a real caller, and export only the supported interface (`unreachable_pub`).
Why: every public item is a promise to keep.
Check: lint

### BP-MOD-008: Keep internal seams internal

Seams a module uses for its own tests stay private; do not expose them through the interface because tests use them.
Why: the interface stays small.
Check: review

## Seams and adapters

### BP-MOD-009: Add a seam only when two adapters exist

Introduce a trait or port only when at least two implementations are justified (production plus a test fake counts); until then write a plain function.
Why: one adapter is a hypothetical seam that only adds indirection.
Check: review

### BP-MOD-010: Accept dependencies, do not create them

Take clients, stores, clocks and configuration as parameters instead of constructing them inside the logic.
Why: the logic becomes testable with a fake and reusable with another source.
Check: review

```rust
/// Plays the next queued item using the given player and clock.
fn play_next(queue: &mut Queue, player: &impl Player, clock: &impl Clock) -> Result<(), PlayError> {
    // ...
}
```

### BP-MOD-011: Return results instead of performing effects

Prefer functions that compute and return a value; push I/O to the edges and keep the middle pure.
Why: pure code is tested with plain values and reused anywhere.
Check: review

### BP-MOD-012: Classify each dependency

Classify each dependency as in-process, local-substitutable, remote but owned, or truly external, and pick the seam and test double from the class (none, a local stand-in, a port with an in-memory adapter, a port with a fake).
Why: the class decides whether a port is needed at all.
Check: review

### BP-MOD-013: Wire adapters at the composition root

Construct concrete adapters once, at the entry point, and hand them in; tests build their own root with fakes.
Why: callers need no provider knowledge.
Check: review

### BP-MOD-014: Choose the adapter at build or start time

Select an implementation from configuration once at start-up or build time, never per call from a runtime value.
Why: per-call selection hides behaviour from caches, logs and tests.
Check: review

### BP-MOD-015: Build for one provider until the second arrives

Write one adapter, no registry or plugin system, until a second provider is actually needed.
Why: abstractions built for a switch that never comes cost more than the switch.
Check: review

### BP-MOD-016: Record accepted couplings

When code deliberately couples to a provider's behaviour, add a line to the governing spec's list of accepted couplings; an agent adding a coupling adds the line.
Why: known couplings can be found and removed later; hidden ones cannot.
Check: review

### BP-MOD-017: Split a crate for a dependency or ownership boundary

Create a new crate or package only for a separate dependency set, a separate owner, or code two workspaces use today ([[best-practices/code-organization]]).
Why: each split freezes an interface and adds build wiring.
Check: review

## State and data

### BP-MOD-018: Store the source, derive the rest

Keep one source of truth per fact and compute derived values when read, instead of storing a second copy that must be kept in sync.
Why: copies drift.
Check: review

### BP-MOD-019: Group state with behaviour only for lifecycle

Use plain data and functions by default; bundle state with methods only when the value has a lifecycle or identity of its own.
Why: most behaviour is a transformation, not an object.
Check: review

## Domain language and decisions

### BP-MOD-020: Resolve ambiguous terms with a concrete case

When a term is vague or overloaded, test it against a concrete edge case, settle the meaning, and record it in `docs/glossary.md` without implementation detail.
Why: callers and docs then agree on what the word means.
Check: review

### BP-MOD-021: Check stated behaviour against the code

When someone states how something works, check the code; surface any contradiction before building on either.
Why: designs built on a wrong belief fail late.
Check: review

### BP-MOD-022: Write an ADR only for a real trade-off

Record a decision in `docs/adr/` only when it is hard to reverse, surprising without context and the result of a genuine trade-off.
Why: ADRs that record the obvious bury the ones that matter.
Check: review

### BP-MOD-023: Design important interfaces twice

For an interface many callers will use, sketch at least two radically different shapes and compare their depth, locality and seam placement before choosing.
Why: the first idea is rarely the best.
Check: review

### BP-MOD-024: Read reference code for shape, never copy it

Read existing code that solved a similar problem for its structure; write the code for this repository rather than copying files.
Why: a copy drags in another project's version pins and workarounds.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (module interfaces, traits, composition roots)

## References

- [Codebase design](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/SKILL.md)
- [Deepening](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/DEEPENING.md)
- [Design it twice](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/DESIGN-IT-TWICE.md)
- [Domain modeling](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/domain-modeling/SKILL.md)
