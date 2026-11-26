---
description: Framework-neutral rules for where UI state lives, when an effect is allowed, how subscriptions are disposed, and how async data and loading states behave.
tags: [best-practices, ui, state]
---

# UI state, effects and async data

Holds for any UI framework. Component rules are in [[best-practices/ui.rules]]; client query caches in
[[best-practices/cache-invalidation.rules]].

## Where state lives

### BP-STATE-001: Put each kind of state in its bucket

Server data comes from a trusted-side read or a query cache; shareable state lives in the URL; cross-tree UI state in a context near its consumers; local UI state in the closest component; derived values are computed.
Why: each bucket has the lifetime its state needs.
Check: review

### BP-STATE-002: Keep shareable state in the URL

Filters, page, tab, sort and search live in the URL, parsed by shared parsers with defaults that are never written into the URL.
Why: reload, back and share keep the view.
Check: review

### BP-STATE-003: Keep transient state out of the URL

Open or closed, hover and in-progress drafts stay in component state.
Why: transient state in the URL pollutes history and links.
Check: review

### BP-STATE-004: Add no global store without a reason

Add a global state store only with a written reason in the governing doc.
Why: the buckets above cover almost every case, and a store hides who depends on what.
Check: review

### BP-STATE-005: Provide context near its consumers

Mount a context provider around the subtree that reads it; make it app-wide only for app-wide state.
Why: a root provider re-renders everything and hides its dependants.
Check: review

### BP-STATE-006: Read shared values where needed instead of drilling

When an accessor exists for a value (locale, theme, session), call it where needed rather than passing it down more than one level.
Why: drilled props couple every intermediate component.
Check: review

### BP-STATE-007: Keep per-user state out of module scope

Module scope holds constants and definitions only, never per-user or per-request state.
Why: on a shared process, module state leaks between users.
Check: review

## Derivations and effects

### BP-STATE-008: Derive instead of copying

Compute display values from their inputs; never copy one piece of state into another with an effect.
Why: a copied value lags its source by one update.
Check: review

### BP-STATE-009: Memoise a derivation only when it pays

A derived value is a plain function by default; cache it only when several readers share it, it is expensive, or its equality check stops further updates.
Why: a memo is a node to maintain, free only when it pays.
Check: review

### BP-STATE-010: Keep render and derivations pure

Rendering and derivation code reads inputs and returns output; it never writes state or performs I/O.
Why: the framework may run it in any order and any number of times.
Check: review

### BP-STATE-011: Use effects only where state leaves the UI

Write an effect only to synchronise with something outside the UI (a canvas, storage, a player, a third-party instance), with its dependencies read visibly in one place.
Why: effects inside the UI create loops and lag.
Check: review

### BP-STATE-012: Assume writes land batched

Assume a state write is visible only after the current synchronous work ends; use the updater form when the next value depends on the previous one.
Why: batched writes read back as old values.
Check: review

### BP-STATE-013: Dispose everything with its owner

Every listener, timer, subscription, animation and pending request is released when its owner is removed; tests fail when one is left running.
Why: leaked work keeps firing and leaks memory.
Check: test

### BP-STATE-014: Create long-lived work in its owner

Create effects and subscriptions in the component that owns them, never inside an event handler.
Why: work created in a handler has no owner to dispose it.
Check: review

### BP-STATE-015: Subscribe to external stores narrowly

Subscribe to an external store inside an owner, select only the values shown, and skip updates whose value did not change.
Why: a broad subscription re-renders on every unrelated change.
Check: review

### BP-STATE-016: Create imperative instances once

Create an imperative library instance once after mount, update it from an effect that reads every input, destroy it on teardown, and guard an async import with a disposed flag.
Why: the instance follows its inputs and never outlives its owner.
Check: review

### BP-STATE-017: Prefer uncontrolled inputs

Use uncontrolled components with default values until the app needs the value; when it does, sync app state through the component's change callbacks, never through click or key handlers.
Why: an agent's or keyboard's change fires the change callback but not a click.
Check: review

### BP-STATE-018: Keep animation time out of domain state

Drive frequently changing visual values (positions, progress) locally or through a style variable; domain state holds only known states.
Why: per-frame state updates re-render everything that reads it.
Check: review

## Async data

### BP-STATE-019: Fetch at the narrowest common owner

Read data at the closest owner shared by its readers; place interactivity at the leaves.
Why: shared data is read once, and interactive code stays small.
Check: review

### BP-STATE-020: Read prepared data on the trusted side

Never fetch in a client effect what the trusted side could read and pass in.
Why: client fetches add a round trip and a loading state.
Check: review

### BP-STATE-021: Keep client-only code at the leaves

Mark only the leaf that needs state, effects, handlers or platform APIs as client-side; pass trusted-side content into interactive shells as children or slots, and pass values a parent already has instead of making a component client-side to fetch them.
Why: every client-side module ships to every user.
Check: review

### BP-STATE-022: Touch platform APIs only after mount

Platform APIs (window, storage, media queries) are read in effects, after mount, or in client-only leaves.
Why: the first render must match what the trusted side produced.
Check: review

### BP-STATE-023: Make the first paint final

The initial render and the hydrated state agree, and sizes and positions are in the first markup.
Why: anything else shifts after load.
Check: test

### BP-STATE-024: Define every async state of a region

Each independently recoverable region defines its loading, pending, empty, partial and failed states, with a local retry.
Why: an undefined state renders as a blank or a crash.
Check: review

### BP-STATE-025: Keep content visible while refreshing

During a refresh, keep showing the previous content with a pending indicator instead of a blank.
Why: the user keeps their place.
Check: review

### BP-STATE-026: Show pending only after a hold

Show a pending indicator only when the wait exceeds a short hold (about 120 ms); a fast change shows nothing.
Why: flashing indicators on fast actions feel slower than none.
Check: review

## Covers

- `apps/`, `packages/` (UI state, effects and data loading)

## References

- [Async UI and agent diagnostics](https://www.solidjs.com/blog/solid-2-0-the-long-hold/)
