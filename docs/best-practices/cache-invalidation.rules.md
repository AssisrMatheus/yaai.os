---
description: How cached reads are tagged and flushed, how change notifications are verified and mapped, and how a UI query cache defines, invalidates and updates queries.
tags: [best-practices, caching]
---

# Cache invalidation

Ids continue the `BP-CACHE` series from [[best-practices/caching.rules]].

## Tags

### BP-CACHE-031: Tag composed output with everything it renders

An output cached as a whole (a screen, a panel, a page) carries the tags of every read it renders, not only its own.
Why: an untagged outer entry keeps showing old data after the inner read was flushed.
Check: test

### BP-CACHE-032: Tag a listing after what it lists

Tag a list or filter result after the item types and categories it contains, never after the list itself.
Why: saving an item must refresh every list that shows it.
Check: review

### BP-CACHE-033: Keep the tag vocabulary in one module

Build tags only through functions in one module per domain, keyed by source ids, never by paths or slugs.
Why: a tag spelled differently in two places never matches.
Check: review

### BP-CACHE-034: Give every tag a flusher

Each tag a read attaches is flushed by at least one change handler, and the strings match byte for byte.
Why: a tag nothing flushes is stale forever.
Check: test

### BP-CACHE-035: Test the tag contract

A test lists each cached read with every change that alters it and asserts that the change's flush intersects the read's tags; it also asserts that unrelated changes flush nothing. A new read adds a row; a new event adds a case.
Why: a missing flush is invisible in a diff and shows up days later as stale data.
Check: test

### BP-CACHE-036: Let reads report what they resolved

When the source can report which records a read touched, attach those keys as tags (within a per-entry budget, collapsing the overflow into a broader tag), only on entries, not on lists.
Why: changes nobody declared as dependencies still reach the read, and overflow flushes more often, never less.
Check: review

## Flushing

### BP-CACHE-037: Flush by reach

What a change names expires now; broad catch-all tags go stale-while-revalidate.
Why: the person who made the change sees it at once without refetching everything.
Check: review

### BP-CACHE-038: Flush by tag, not by path

Invalidate tags, never whole paths or screens.
Why: a path flush discards unrelated reads on that path and refetches all of them.
Check: review

### BP-CACHE-039: Flush the minimum

Flush an index or category only when membership changes (an item appears, disappears or moves), not on every save.
Why: over-flushing turns every edit into a burst on the source.
Check: review

### BP-CACHE-040: Flush broadly on an unknown event, never nothing

A change notification the mapping does not model triggers a broad stale flush and a log line; it is never rejected with an error and never flushes nothing.
Why: an unmodelled change must still reach readers.
Check: test

### BP-CACHE-041: Keep the event mapping pure

Map a notification payload to the set of tags to flush in a pure function the adapter owns, parsed leniently ([[best-practices/data-boundaries.rules]]).
Why: the mapping is tested with plain payloads.
Check: test

### BP-CACHE-042: Scope payload facts to their event

Interpret a payload field in the context of the event that carries it; the same field can mean a site-wide change in one event and nothing in another.
Why: a context-free reading over- or under-flushes.
Check: review

### BP-CACHE-043: Verify notification senders

Authenticate a change notification with a shared secret compared in constant time, and tolerate duplicate and out-of-order delivery.
Why: anyone can call an open flush endpoint, and senders retry.
Check: test

### BP-CACHE-044: Warm after a flush on a best-effort basis

Refetching after a flush never makes the notification fail.
Why: the flush already happened; a warm-up error must not trigger a sender retry storm.
Check: review

### BP-CACHE-045: Check both sides of an integration

A change to how the app reads or reacts to a system the project also maintains is checked against that system in the same change, or the change says what the other side needs.
Why: half of an integration change breaks the other half.
Check: review

## Client query caches

### BP-CACHE-046: Define each query once

Define each query as one factory holding its name, normalised arguments, key and fetch function, used by prefetch, reads and invalidation alike.
Why: prefetched data then hydrates without a refetch and invalidation hits the same key.
Check: review

### BP-CACHE-047: Invalidate through the factory's key

Invalidate with the key the factory produces, never a re-typed key literal.
Why: a copied literal drifts from the real key.
Check: review

### BP-CACHE-048: Keep query factories free of closures

A parameterised query takes its parameters as arguments and never closes over component state.
Why: closed-over state is missing from the key.
Check: review

### BP-CACHE-049: Use one query client per scope

Create one query client per request on a server and one per process in a UI, never inside a component's render.
Why: a client recreated on render loses its cache.
Check: review

### BP-CACHE-050: Query only interactive or per-user data on the client

Content the trusted side can prepare is read there; a client query is for data that changes with interaction or per user.
Why: a client query ships a library and a loading state for content that could arrive ready.
Check: review

### BP-CACHE-051: Fail queries by error

A query function signals failure by returning `Err` or throwing, never by resolving to an error value.
Why: the cache enters its error and retry state only on a failure.
Check: review

### BP-CACHE-052: Gate dependent queries on valid input

A query that needs a previous result is disabled until that input is valid, instead of guarding with early returns in the parent.
Why: the dependency stays declared in one place.
Check: review

### BP-CACHE-053: Keep the previous page while paging

Show the previous page's items while the next page loads.
Why: the list does not flash empty between pages.
Check: review

### BP-CACHE-054: Roll back optimistic updates

An optimistic change snapshots the current value, applies the change, rolls back on failure and reconciles with the authoritative value when settled.
Why: a failed write must not leave a lie on screen.
Check: test

### BP-CACHE-055: Invalidate what a mutation changes

Every mutation invalidates or updates the queries whose data it changes.
Why: otherwise the screen shows the old value after a successful write.
Check: test

## Covers

- `apps/`, `packages/`, `tooling/` (tag vocabularies, change handlers and query factories)
