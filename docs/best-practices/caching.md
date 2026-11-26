---
description: Where a cache sits, what its key holds, how long entries live, and what a cached read does when its source fails or is slow.
tags: [best-practices, caching]
---

# Caching

Rules for any cache: in memory, on disk, shared between processes, or in a UI query client. Invalidation and change
notifications are in [[best-practices/cache-invalidation]]; tool caches also follow [[best-practices/tooling-cli]].

## Placement

### BP-CACHE-001: Use one caching model per app

An app uses one cache mechanism for a given kind of read; two mechanisms keep separate entries that go stale apart.
Why: one invalidation must reach every copy.
Check: review

### BP-CACHE-002: Cache each read once, in one layer

Cache external reads in one cache layer per domain that wraps the adapter behind the same interface; the adapter never caches and the service stays uncached.
Why: the cache layer is the one place audited for keys and invalidation, and the service can turn failures into answers without caching them.
Check: review

### BP-CACHE-003: Give every cached read an owner, lifetime, trigger and budget

Each cached read states, at the cache site, who owns it, how long entries live, which event invalidates them and how large they may grow.
Why: a cache without these decisions grows stale or unbounded.
Check: review

### BP-CACHE-004: Share caches across instances when the app runs several

When several processes serve the same data, use a shared cache, or make sure invalidation reaches every instance.
Why: a per-instance cache protects nothing for the next instance and ignores an invalidation sent to another.
Check: review

### BP-CACHE-005: Respect per-entry size limits

Know the store's maximum entry size; split or exclude a read that can exceed it, with a comment saying why.
Why: oversize writes often fail silently and become a permanent miss.
Check: review

### BP-CACHE-006: Fetch shared data once at its owner

Read data many children need (menus, settings, profiles) once at their common owner and pass it down, instead of a cached read per child.
Why: each extra cache scope needs its own keys and invalidation.
Check: review

### BP-CACHE-007: Dedupe within one request by a stable string key

Deduplicate repeated reads within one request or command with a key built from a stable string, not from a freshly built collection.
Why: a key built from a new array or object never matches.
Check: review

## Keys

### BP-CACHE-008: Put every varying dimension in the key

The key includes everything the result depends on: locale, principal, filters, schema version and build-or-run phase.
Why: a dimension missing from the key serves one user's or one locale's result to another.
Check: test

### BP-CACHE-009: Pass key dimensions as explicit arguments

A cached function receives every key dimension as a plain positional argument; it never reads one implicitly (from ambient context, globals or closure state).
Why: the key stays visible in review and works from every caller.
Check: review

### BP-CACHE-010: Build keys from order-stable values

Never key a cache with a structure whose serialised order can vary (an unordered map, an object with optional fields); flatten queries into positional values.
Why: two equal queries must produce one key.
Check: review

### BP-CACHE-011: Keep credentials out of keys

Never pass a token or credential as a cache key component; credentialed and draft reads bypass shared caches.
Why: keys are logged, stored and shared.
Check: review

### BP-CACHE-012: Bypass shared caches for private data

Per-user or private results stay out of shared caches unless isolation by principal is proven by a test.
Why: one user's data must never reach another.
Check: test

## Failures

### BP-CACHE-013: Return an error from a cached read on transient failure

A cached reader returns `Err` on a transport failure; it never stores `None`, an empty list or a "not found" for a failure.
Why: a stored failure is served as missing content for the whole lifetime.
Check: test

### BP-CACHE-014: Cache only successful values

Cache only successful values; errors pass through uncached.
Why: error types lose their identity through serialisation.
Check: review

### BP-CACHE-015: Distinguish a miss from an outage

Keep "the source says it does not exist" separate from "the source could not answer" all the way to the caller.
Why: the caller shows "missing" for one and "try again" for the other.
Check: test

### BP-CACHE-016: Keep the last healthy value on a failed refresh

On a failed refresh, keep the last good value instead of overwriting it with an error or a fabricated empty result.
Why: a blip in the source must not blank the screen.
Check: test

### BP-CACHE-017: Serve the last good copy when the source fails or hangs

Keep a last-good copy outside the expiring entry, served when the source errors or misses a soft deadline, with a hard timeout on the request; while serving it, shorten the entry's lifetime and label it stale.
Why: users never see the outage, and the entry heals soon after the source returns.
Check: test

### BP-CACHE-018: Fail a build when its source is down

A build that prerenders data fails when the source is unreachable instead of shipping empty output.
Why: an empty deployment replaces a working one.
Check: review

### BP-CACHE-019: Degrade a prebuilt set on outage, never to nothing

A build-time list of items to prepare (paths, thumbnails) falls back to the minimal set on a source outage and is never empty.
Why: the rest is produced on demand once the source returns.
Check: review

### BP-CACHE-020: Keep junk requests away from cached reads

Reject scanner probes and malformed identifiers before any cached read, with a narrow guard that never matches a real item.
Why: each junk key otherwise creates an entry nothing ever invalidates.
Check: test

## Lifetimes and load

### BP-CACHE-021: Invalidate by event when the source can notify

For a source that sends change notifications, cache without a time floor and invalidate on the event ([[best-practices/cache-invalidation]]).
Why: a time floor expires everything at once and stampedes the source.
Check: review

### BP-CACHE-022: Check the composed lifetime

When an output combines several cached parts, the shortest lifetime wins; check the result's effective lifetime after adding a part.
Why: one short-lived part drags the whole output to its lifetime.
Check: review

### BP-CACHE-023: Stagger expiry and coalesce misses

Entries written together expire together; stagger lifetimes and coalesce concurrent misses into one source request.
Why: synchronised expiry becomes a herd of requests at the source.
Check: review

### BP-CACHE-024: Keep the source under its load threshold

Limit build-time concurrency, prefetch on intent rather than on visibility, and add no default retries on server-side transports.
Why: bursts trip rate limits and firewalls, which takes every uncached read down.
Check: review

### BP-CACHE-025: Disable lower caches under a tagged cache

Turn off URL-level or HTTP caches below a cache that is invalidated by events, and avoid unpurgeable time-to-live edge caches unless a measured load problem requires one; then vary its URL per time window.
Why: a lower cache that ignores invalidation serves stale data after a flush.
Check: review

### BP-CACHE-026: Plan for a cold cache after deploy

When keys include a build or deployment id, entries do not survive a deploy; size the source for the first requests after it.
Why: a cache assumed warm after deploy hides a cold burst.
Check: review

### BP-CACHE-027: Prebuild a bounded set

Prepare a bounded, most-used subset at build time and produce the rest on first request.
Why: prebuilding everything lengthens builds and bursts the source.
Check: review

### BP-CACHE-028: Keep no-store for per-user data

Disable caching only for per-user or per-request data, and keep a minimum revalidation interval of seconds, not milliseconds, elsewhere.
Why: blanket no-store or tiny intervals turn every view into source load.
Check: review

### BP-CACHE-029: Verify caching on a production build

Check caching against a production build: a warmed entry is served, a change plus its notification makes the next read fresh, and a second pass makes zero source requests.
Why: development servers do not cache the way production does.
Check: test

### BP-CACHE-030: Watch hit rate and entry size

After adding a cached read, check its hit rate and entry sizes in the store's metrics.
Why: a cache that never hits or entries near the limit are invisible otherwise.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (cache layers, cached readers and tool caches)
