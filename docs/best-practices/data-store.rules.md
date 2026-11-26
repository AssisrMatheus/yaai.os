---
description: How code shapes, migrates, queries and writes a data store, stated stack-neutral, for when a store and its data-model spec exist.
tags: [best-practices, data, store]
---

# Data store

No data store is chosen yet. These rules apply from the first change that adds one: a database, an embedded store, or
any store that keeps data across runs. Its data-model spec will live in [[specs/SKILL]]; the spec decides names, ids
and indexes, and these rules decide how code follows it. Order of priority: correct, then fast, then readable; when the
fast form is less readable, take it and comment why. Validation and adapters are in [[best-practices/data-boundaries.rules]],
caching in [[best-practices/caching.rules]], connection helpers in [[best-practices/security.rules]].

## The store and its services

### BP-STORE-001: Pass the store handle to the service

A service receives the store handle (a pool, a connection, an embedded store) as an argument from the composition root and owns every query and transaction made through it; no code reads a global handle.
Why: the service is tested against a fresh store and every query has one owner ([[best-practices/module-design.rules#BP-MOD-010: Accept dependencies, do not create them]]).
Check: review

```rust
/// Lists the titles of a library, newest first.
pub async fn list_recent(store: &Store, library: LibraryId, page: Page) -> Result<Vec<Title>, ListError> {
    // ...
}
```

### BP-STORE-002: Keep queries out of transport adapters

A command, handler or IPC endpoint calls a service; it holds no query, no transaction and no store handle.
Why: a query in an adapter skips the service's validation and transactions and is copied by the next entry point.
Check: review

### BP-STORE-003: Write only the tables your service owns

A service writes only the tables of its own namespace; data in another namespace changes through that namespace's service.
Why: each table has one writer to reason about.
Check: review

## Schema and migrations

### BP-STORE-004: Match the data-model spec

Every table, column, index, view and stored type matches the data-model spec; a structure the spec does not hold is a spec change the owner decides first, then code. A new entity the spec has not reached yet follows the spec's existing patterns, and the change adds it to the spec.
Why: docs are the source of truth (INV-DOCS-001); a schema that drifts from its spec is trusted wrongly.
Check: review

### BP-STORE-005: Change the schema only through migrations

Every schema change is a new, ordered migration committed with the spec change and the code that needs it; an applied migration is never edited.
Why: every copy of the store reaches the same schema by replaying the same steps.
Check: test

### BP-STORE-006: Name schema items by the spec's convention

Name tables, columns and keys by the convention the data-model spec fixes; unless it says otherwise: `snake_case`, singular table names, `<table>_id` references, `is_`/`has_` booleans and `_at` timestamps.
Why: a name then tells meaning and type without opening the schema.
Check: review

### BP-STORE-007: Type columns for the value they hold

Store instants as time-zone-aware timestamps, counts and money as integers in minor units, exact ratios as exact decimals, human-unique names in a case-insensitive form, and structured documents only for a cold tail of fields no query filters on.
Why: the type keeps invalid values out and lets the store compare and index correctly.
Check: review

### BP-STORE-008: Mint time-ordered ids, keep source ids

An id the program mints follows the spec's id strategy, a time-ordered unique id by default; a row imported from a source keeps the source's id ([[best-practices/data-boundaries.rules#BP-DATA-015: Keep the source's ids]]).
Why: time-ordered ids keep index inserts local and leak no global count; source ids keep imports re-runnable.
Check: review

### BP-STORE-009: Model a growing vocabulary as a lookup table

A set of values that can grow (kinds, genres, sources) is a lookup table referenced by id; a store-native enum holds only a closed internal vocabulary.
Why: adding a value is then a row, not a schema change that locks the table.
Check: review

### BP-STORE-010: State invariants as constraints

Primary keys, foreign keys, `NOT NULL`, unique business keys and check constraints state the invariants the spec gives; application code does not stand in for them.
Why: a constraint holds for every writer, including a buggy one.
Check: review

### BP-STORE-011: Index every reference and every hot access pattern

Index every foreign-key column, and give each hot query the index the spec names for it (partial, expression or covering).
Why: an unindexed reference turns joins and cascading deletes into full scans.
Check: review

### BP-STORE-012: Point references toward stable data

A table references only tables at least as durable as itself: disposable data may reference durable data, never the reverse.
Why: clearing disposable data then never breaks a durable row.
Check: review

### BP-STORE-013: Give each table a stated lifecycle

The spec says for each table whether rows are soft-deleted (a `deleted_at`), cascade away with their parent, or are permanent; code deletes accordingly.
Why: a query can then tell "deleted" from "never existed".
Check: review

## Reads

### BP-STORE-014: Qualify every table by its namespace

When the store has namespaces, every query names the table's namespace; nothing relies on a search path.
Why: an unqualified name can resolve to another namespace's table.
Check: review

### BP-STORE-015: Read a derived value through its one definition

When the spec defines a derived read (a view merging a source with local overrides, a resolved setting), every query reads through it and none re-derives it by hand.
Why: the derivation changes in one place instead of drifting in each copy.
Check: review

### BP-STORE-016: Filter soft-deleted rows

Every read of a soft-deleted table excludes deleted rows, with the exact predicate its partial index was built on.
Why: a deleted row otherwise reappears, and a reworded predicate stops the planner using the partial index.
Check: review

### BP-STORE-017: Write sargable predicates

Compare a bare column with a value; when a computed predicate is needed, index that expression; query a time range as `>= start AND < end`, never by truncating the column.
Why: a function around a column hides it from its index, so every row is evaluated.
Check: review

### BP-STORE-018: Shape composite indexes as equality, range, sort

Order a composite index's columns as equality, then range, then sort, and write the query's predicates and ordering in the same shape.
Why: a query that matches only a prefix of the index uses only that prefix.
Check: review

### BP-STORE-019: Select only the used columns on a hot path

A hot query selects exactly the columns its caller uses, matched to a covering index when one exists; selecting every column is for cold paths only.
Why: a narrow select on a covering index never touches the table rows.
Check: review

### BP-STORE-020: Read sets, never one row per parent

Fetch the children of many parents with one query (a membership list or a join) and group the flat result in code; never query once per parent in a loop.
Why: N+1 queries cost N round-trips where one would do.
Check: review

```rust
// Fetch every library's titles in one query, then group by library.
let titles = titles::for_libraries(store, &library_ids).await?;
let by_library = group_by(titles, |title| title.library_id);
```

### BP-STORE-021: Name intermediate results

A read that needs an intermediate result uses a named common table expression or a view, not a nested subquery repeated in the outer query.
Why: the store and the reader both reason about a named step on its own.
Check: review

### BP-STORE-022: Read a hot query's plan before calling it done

Before a hot-path query counts as done, read its plan on representative data volume and fix sequential scans of large tables, nested loops with a large inner side, and sorts feeding a limit; put the plan's verdict in the change.
Why: a query that looks right was not verified until the store said how it runs it.
Check: review

### BP-STORE-023: Build queries with the typed layer

Build queries with the project's typed query layer; write raw query text only for a shape it cannot express, in one function, with a comment saying why.
Why: the typed layer checks names and types against the schema; raw text is checked only at run time.
Check: review

### BP-STORE-024: Read pinned versions inside a running session

A long-running session reads definitions and settings at the version it pinned when it started, never the live current state.
Why: a change made mid-session otherwise alters behaviour halfway through it.
Check: review

## Writes

### BP-STORE-025: Resolve conflicts on the batch, not with a read per row

Express "skip or update what exists" as the store's conflict clause on the batched statement, never as an existence read before each insert.
Why: the re-run stays a no-op in the same round-trip instead of doubling the queries.
Check: review

### BP-STORE-026: Update many rows from one values list

Apply different values to many rows with one statement that joins a values list to the table; for two or three rows, separate statements in one transaction are fine.
Why: one statement replaces a loop of single-row updates.
Check: review

### BP-STORE-027: Chain a fixed-shape dependent write in one statement

When a fixed, known chain of writes needs an earlier row's id (a parent then its children), write it as one statement that feeds each step the previous step's returned ids, where the store supports it; when the chain branches, loops or spans existing service calls, use a transaction instead.
Why: one statement is one round-trip and atomic; a chain grown branchy reads better as a transaction.
Check: review

### BP-STORE-028: Make seeds idempotent

A seed writes in bounded batches with conflict clauses, so running it twice leaves the store unchanged.
Why: seeds rerun on every fresh environment and every retry.
Check: test

## Transactions

### BP-STORE-029: Wrap all-or-nothing writes in a transaction

Use a transaction when several statements must succeed or fail together, when a write depends on a read of a row others can change, or when an idempotency key is stored with its effect.
Why: a crash between statements otherwise leaves a partial write.
Check: review

### BP-STORE-030: Run every statement on the transaction handle

Inside a transaction, every query uses the transaction's handle, and functions called inside take that handle, never the pool.
Why: a query on the pool runs outside the transaction, sees none of its writes and never rolls back.
Check: review

### BP-STORE-031: Keep transactions short

Put only the statements of the atomic outcome inside a transaction: no calls to other systems, no unrelated reads, no long computation; resolve inputs before it and act on the result after commit.
Why: a transaction holds locks for its whole duration, and every writer to those rows waits.
Check: review

### BP-STORE-032: Guard contended rows with a revision

A row several writers update carries a revision; a write compares and increments it in the same statement, and a mismatch returns a conflict error.
Why: concurrent writers then fail visibly instead of overwriting each other.
Check: test

### BP-STORE-033: Use a savepoint for an optional step

When one step inside a transaction may fail without aborting the rest, roll back to a savepoint; never nest transactions, and default to failing together.
Why: a savepoint is the store's real partial rollback; nested transactions are not.
Check: review

### BP-STORE-034: Store an idempotency key with each once-only effect

A write that must happen once (a credit, a purchase, a delivered notification) stores an idempotency key, unique in the schema, in the same transaction as its effect.
Why: a retried request then finds the key and does nothing.
Check: test

## Covers

- `apps/`, `packages/`, `tooling/` (store handles, schema, migrations, seeds, queries and transactions)
