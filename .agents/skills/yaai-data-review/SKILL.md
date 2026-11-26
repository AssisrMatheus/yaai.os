---
name: yaai-data-review
description: Review of data-store work against its data-model spec and the BP-STORE rules, once a store exists. Use before adding or changing a table, column, index, migration or seed, or the types that mirror rows; before writing or reviewing a query, a transaction or a service method that reads or writes the store; and when a loop calls the store once per item.
---

# Data review

Check data-store work in two parts: the **structure** (what the schema holds) against the data-model spec, and the
**queries and writes** against `docs/best-practices/data-store.rules.md`. Both parts can apply to one change: a
migration that adds a table and the seed that fills it. The spec and the rules page are the authority; read them every
time and cite them by heading and rule id.

## 1. Check that it applies

No data store is chosen yet. This skill applies once a store exists and its data-model spec is listed in
`docs/specs/SKILL.md` (pages `docs/specs/<name>.spec.md`).

- No spec yet, and the change adds the first store or a new one: the spec comes first. Stop, tell the user, and
  write it with the `yaai-doc-change` skill once they decide.
- The change is pure in-memory logic that never reaches the store: this skill does not apply.

Done when you know the spec pages that govern the change, or have stopped.

## 2. Read the authority

Read `docs/best-practices/data-store.rules.md` in full, the data-model spec page of every namespace the change
touches, and `docs/best-practices/data-boundaries.rules.md` when the change adds or reshapes an adapter. Done when
each changed table, query and write is tied to a spec heading or a rule.

## 3. Check the structure

For each table, column, index, view, stored type, migration, and the Rust or TypeScript type that mirrors a row,
confirm the rules of `Schema and migrations` (BP-STORE-004 to BP-STORE-013) and the spec's decisions on namespace,
ids, names, types, vocabularies, constraints, indexes, reference direction and lifecycle. Done when every structural
item is matched to the spec or recorded as a deviation or a gap (step 5).

## 4. Check the queries and writes

Find every store call in the diff; a store call inside a loop is the strongest signal, almost always an N+1 read or a
row-at-a-time write. Confirm the rules of `The store and its services`, `Reads`, `Writes` and `Transactions`
(BP-STORE-001 to BP-STORE-003, BP-STORE-014 to BP-STORE-034) and BP-DATA-037 for batching. A claim that a hot query is
fast needs its plan (BP-STORE-022). Done when every store call is matched to the rules it must follow.

## 5. Deviation or gap

- **Deviation**: the spec or a rule decides this and the change contradicts it (a per-row insert loop, a query on the
  pool inside a transaction, an id minted against the spec's strategy). Do not build it. Name the conflict with the
  rule id or spec heading, state the cost the rule prevents, show the conforming form, and ask the user whether they
  mean to change the spec. Default to the conforming form while waiting. A spec that is wrong changes first, through
  `yaai-doc-change`, never as a one-off exception in code.
- **Gap**: the spec has not modelled this entity, but its patterns clearly extend to it (a new table shaped like its
  neighbours). Extend the patterns faithfully, and report the addition so the spec absorbs it. When unsure whether it
  is a gap or a deviation, treat it as a deviation.

Done when every item is conforming, a reported gap, or a deviation the user has answered.

## 6. Report

Proceed and cite what you relied on (`batched per BP-DATA-037`, `time-ordered id per <spec heading>`). In a review,
report each finding as the `yaai-code-review` skill does, with its rule id or spec heading; findings are blocking.

## Example

```rust
for title in &titles {
    titles::insert(store, title).await?;
}
```

Deviation: one statement per row (BP-DATA-037), not atomic. Conforming form: one batched insert per chunk with a
conflict clause (BP-STORE-025), inside a transaction when the rows must land together (BP-STORE-029). Ask before
keeping the loop; rewrite directly when asked to fix rather than flag.
