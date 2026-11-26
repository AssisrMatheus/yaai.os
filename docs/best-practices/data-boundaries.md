---
description: How code validates input at trust boundaries, layers services and adapters around external systems, models domain data, and reads or writes it.
tags: [best-practices, data]
---

# Data boundaries

Where data enters, where it is trusted, and how an external system is kept behind one adapter. Caching is in
[[best-practices/caching]], secrets and escaping in [[best-practices/security]], error mapping in
[[best-practices/rust-errors]].

## Trust boundaries

### BP-DATA-001: Validate at the trusted entry point

Parse and validate every external input where it enters trusted code: CLI arguments, files, environment, network responses, IPC messages, webhooks and plugin data.
Why: everything behind the entry point can then trust its inputs.
Check: review

### BP-DATA-002: Validate again where the rule lives

A service validates its own inputs even when a caller already did, because services are called from several entry points (commands, handlers, jobs, other services).
Why: a check at one entry point is skipped by the next.
Check: review

### BP-DATA-003: Share one schema per contract

Define each input or wire contract once and use it on every side (UI and core, Rust and TypeScript through generated types or shared fixtures).
Why: two schemas for one contract drift and accept different data.
Check: test

### BP-DATA-004: Treat client-side validation as a convenience

The trusted side re-validates everything a UI or remote client already checked; the client check only gives early feedback.
Why: a client can be bypassed or modified.
Check: review

### BP-DATA-005: Parse event payloads leniently

Parse webhook and event payloads so a malformed optional field reads as absent and the event is still processed; reject only a missing required field.
Why: one bad optional field must not stop a change from being applied.
Check: test

### BP-DATA-006: Authorise at the boundary, per object and action

Check that the caller may perform this action on this object at the trusted boundary, after parsing and before calling the service.
Why: authorisation placed deeper is skipped by the next entry point.
Check: review

### BP-DATA-007: Keep logic away from the transport

Business logic never reads request headers, cookies, URLs or process state; the entry point extracts the principal and inputs and passes them in.
Why: the logic then runs and tests without a request.
Check: review

## Services and transport adapters

### BP-DATA-008: Keep business logic in services

Put business logic in service modules; a transport adapter (a CLI command, HTTP or IPC handler, form action) only parses, calls one service and maps the result.
Why: one place to test the logic, and every entry point behaves the same.
Check: review

### BP-DATA-009: Return the payload across the transport

A transport adapter returns the service's payload to its caller, not the whole internal result wrapper.
Why: callers depend on the data, not on internal plumbing.
Check: review

### BP-DATA-010: Declare service dependencies

A service receives the other services it calls as declared dependencies; it never constructs one inline, and dependencies form no cycle.
Why: the dependency graph stays visible and testable.
Check: review

## External systems

### BP-DATA-011: Put an external system behind a service and adapter

When the code reads or writes an external system whose shapes and quirks would otherwise spread, put it behind an adapter (one module per source) and a service the rest of the code calls.
Why: quirks stay in one folder, logic is testable against a fake, and one cache layer covers every read.
Check: review

### BP-DATA-012: Reach the external system only through its service

Code outside the adapter's module imports only the service and the domain model, never the adapter, its client or its generated types (module privacy or import rules).
Why: one leak makes the seam decorative.
Check: lint

### BP-DATA-013: Let only domain types cross the adapter

No wire type, vendor field name, vendor locale code or raw payload leaves the adapter; it maps everything into the domain model.
Why: the model is what makes a fake, a cache and a provider change possible.
Check: review

### BP-DATA-014: Keep domain values plain data

Domain values are plain serialisable data: no handles, closures, platform objects or class instances; timestamps are normalised to UTC by the adapter.
Why: values pass through caches, IPC and logs unchanged.
Check: review

### BP-DATA-015: Keep the source's ids

Carry the source's own identifiers, as strings or newtypes, rather than inventing local ones.
Why: they become cache keys and invalidation targets that the source also knows.
Check: review

### BP-DATA-016: Normalise unset to absent

The adapter turns a source's "unset" representation (`""`, `0`, empty object) into `None`.
Why: callers then use one absence check instead of guessing each source's convention.
Check: test

### BP-DATA-017: Map vendor enums with a fallback

The adapter maps a vendor's option values to domain enums, falling back to the default when the vendor sends a value the domain does not know.
Why: an option removed or added upstream degrades one field instead of failing a read.
Check: test

### BP-DATA-018: Drop what cannot render

The adapter drops items missing a required part (a link without a label) instead of passing them on.
Why: views never receive half-formed data.
Check: review

### BP-DATA-019: Fail a read on transport failure

An adapter read returns `Err` when the source fails and `Ok(None)` only for a genuine absence; with protocols that return errors as data, it checks for empty roots.
Why: a failure reported as "no content" gets cached and shown as missing.
Check: test

### BP-DATA-020: Read configuration before parsing

An adapter reads the configuration it needs to map data (base URLs, origins) before parsing, so missing configuration fails the read instead of producing wrong values.
Why: a wrong value is cached; a failed read is retried.
Check: review

### BP-DATA-021: Keep source-agnostic logic in the service

Resolving a path to an id, guards, credential verification and combining change notifications live in the service, not in an adapter.
Why: a second source then reuses them unchanged.
Check: review

### BP-DATA-022: Use one client per backend

Each backend gets one client module that carries timeouts, logging and tracing; no code builds its own ad-hoc client.
Why: cross-cutting behaviour is wired once.
Check: review

### BP-DATA-023: Inject auth through the client's extension point

Add headers and credentials through the client's middleware or plugin mechanism, never by reaching into its internals.
Why: internals change between versions.
Check: review

### BP-DATA-024: Scope generated wire types to the adapter

Generate wire types and query documents only from and into the adapter's module, so nothing else can import them.
Why: a generated type used outside the adapter couples that code to the vendor.
Check: lint

### BP-DATA-025: Select only what the domain holds

Each query selects exactly the fields its mapper turns into domain values, and the selection lives beside the mapper.
Why: no over-fetching, and a field change touches one file.
Check: review

### BP-DATA-026: Name operations specifically

Give every query or operation a unique, specific name (`GetRecentTitles`, not `GetData`).
Why: generated code, logs and search depend on unique names.
Check: review

### BP-DATA-027: Use one transport per resource

Fetch a resource through one transport only, so all reads share keys and invalidation.
Why: two transports keep two caches that go stale separately.
Check: review

### BP-DATA-028: Read a resource through one shared selection

All reads of the same resource (normal and preview) use one shared selection definition.
Why: a field added once reaches every read.
Check: review

## Content models

### BP-DATA-029: Model composed content as a tagged union

Represent composed content (blocks, sections, rows) as `{ kind, key, data }` with `kind` a domain slug and one data type per kind; the adapter maps vendor type names to kinds and drops unknown ones with a log line.
Why: renderers depend on the domain, and an unknown kind fails closed.
Check: review

### BP-DATA-030: Render content kinds exhaustively

The renderer maps every content kind to its view through an exhaustive `match` or checked table, so a missing kind is a compile error.
Why: a new kind cannot silently render nothing.
Check: lint

### BP-DATA-031: Let a block carry its own query

A content block that needs more data than its parent read gives it carries a query description in its data and is resolved through a cached service call.
Why: the extra read is cached and invalidated like any other.
Check: review

## Reads and writes

### BP-DATA-032: Page to completion with a cap

A paginated read returns items with a cursor and a has-next flag; a caller that needs everything pages to completion with a maximum and logs a warning when the cap is hit.
Why: silent truncation looks like the whole answer.
Check: test

### BP-DATA-033: Keep one failing source from blanking an aggregate

An output built from several sources handles each source's failure separately and keeps the others.
Why: one outage must not empty a whole listing.
Check: test

### BP-DATA-034: Dedupe aggregates by canonical key

Deduplicate aggregated entries by their final canonical key, first writer wins.
Why: two sources often list the same item under different paths.
Check: test

### BP-DATA-035: Keep timestamps honest

Report each record's real modification time; never stamp every entry with the current time.
Why: consumers use timestamps to decide what to refetch.
Check: test

### BP-DATA-036: Send writes past every cache

Writes and credentialed reads bypass every cache.
Why: a cached write repeats or hides an action.
Check: review

### BP-DATA-037: Chunk bulk writes

Write large imports in chunks (on the order of 500 rows) instead of one write per item in parallel.
Why: per-item writes overload hooks and connection pools.
Check: review

### BP-DATA-038: Use the high-level data API by default

Read and write through the data layer's high-level API; a lower-level bypass says in a comment which guarantees it gives up (hooks, access control, validation, relations, transactions) and why.
Why: each bypass silently drops behaviour the high-level path provides.
Check: review

### BP-DATA-039: Type low-level access from the generated schema

Low-level database access goes through a helper typed from the generated schema, and writes the relation or join rows a parent record needs.
Why: untyped access and missing join rows lose data silently.
Check: review

### BP-DATA-040: Group mixed boolean predicates explicitly

Write combined AND/OR conditions with explicit grouping, especially in access rules.
Why: implicit grouping can turn an AND into an OR and widen access.
Check: review

### BP-DATA-041: Resolve the public origin in one place

Compute the application's base URL in one function from configuration; every absolute URL derives from it.
Why: two origins split links and leak preview hosts.
Check: review

### BP-DATA-042: Answer a real not-found status

A request for a missing resource answers a not-found status decided before any output streams; a source failure is an error, never a not-found.
Why: a "not found" page served as success, or an outage served as "not found", misleads every client and cache.
Check: test

### BP-DATA-043: Check statuses on a production build

Check status codes with a real request against a production build across the lifecycle: unknown, published, removed, renamed and non-canonical paths.
Why: development servers stream and redirect differently.
Check: test

### BP-DATA-044: Compare before and after a data-path refactor

When restructuring how data is read, record each affected command's or route's output, status and cache keys before and compare after.
Why: refactors of the data path break silently otherwise.
Check: test

### BP-DATA-045: Order request middleware

Run a transport adapter's middleware in a fixed order: logging, then authentication, then rate limiting, then domain checks.
Why: every request is logged, and unauthenticated requests never reach rate-limit or domain state.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (entry points, services, adapters and domain models)

## References

- [Codebase design](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/SKILL.md)
- [Domain modeling](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/domain-modeling/SKILL.md)
