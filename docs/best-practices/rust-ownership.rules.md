---
description: How Rust code shapes ownership and borrowing, keeps computation pure, and keeps hot paths allocation-free.
tags: [best-practices, rust, performance]
---

# Rust ownership and hot paths

A borrow error says two pieces of code want the same data at once; the fix is in the shape of the data, not in a
copy that makes one line compile. Ids continue the `BP-RUST` series. Measurement rules are in
[[best-practices/performance.rules]].

## Ownership and borrowing

### BP-RUST-064: Borrow to inspect, own to keep

Take `&str`, `&Path` or `&[T]` to read, an owned value to retain it, and `&mut T` only for an intended in-place change (`clippy::needless_pass_by_value`, `ptr_arg`).
Why: the signature tells the caller who can still use the value.
Check: lint

### BP-RUST-065: Fix borrow conflicts in the data layout

Do not `clone`, `to_vec` or `collect` only so a borrow ends at a brace; change the data layout instead.
Why: the copy costs on every pass and hides the design problem.
Check: review

### BP-RUST-066: Split state into disjoint parts

Split a large state struct into sub-structs and borrow fields apart (`let Self { index, scratch, .. } = self;`); pass a function the fields it uses, not `&mut self`.
Why: disjoint borrows compile without copies, and signatures state the data flow.
Check: review

### BP-RUST-067: Write subsystems as functions over their data

Implement each subsystem as free functions that take the sub-structs they read and write, not as one more `impl` block on a struct that holds everything.
Why: every method on a god struct borrows all of it, so every pair of subsystems conflicts.
Check: review

### BP-RUST-068: Check out scratch storage only

Move a buffer out with `mem::take` only when it holds no live state, nothing called meanwhile reads the emptied field, and every exit path (including `?`) returns it.
Why: taking live state leaves readers looking at an empty field.
Check: review

### BP-RUST-069: Use ids into flat storage, not shared pointers

Keep state as typed ids into `Vec`s owned by one struct; give reusable slots a generation so stale ids fail. No `Rc<RefCell<T>>` or back-references in state.
Why: runtime borrow panics and pointer chasing disappear, and state stays plain data.
Check: review

### BP-RUST-070: Fix lifetime errors in the ownership

Do not use `'static`, `Box::leak` or `unsafe` to make a lifetime error go away; fix the ownership.
Why: the compiler stops checking exactly what it warned about.
Check: review

### BP-RUST-071: Gather, then apply

When a pass must read and write the same collection, collect decisions into a reused scratch buffer, then apply them in order.
Why: two phases replace a full copy and keep the order of effects.
Check: review

### BP-RUST-072: Keep one owner of mutable state

Give each piece of mutable state one owner; share immutable data with `Arc` only across real owners, and document the invariant behind any lock or interior mutability.
Why: shared mutable state is where concurrency bugs live.
Check: review

### BP-RUST-073: Report a restructure instead of cloning past it

When a borrow conflict can only be fixed by restructuring state outside the task's scope, stop and report which borrows conflict; the restructure becomes its own task.
Why: a quick clone ships the design problem with a cost attached.
Check: review

### BP-RUST-074: Let the caller supply storage

Return an iterator, accept a slice or a caller-owned buffer when the caller can reuse storage, instead of returning a fresh `Vec` per call.
Why: the caller controls allocation.
Check: review

## Pure computation

### BP-RUST-075: Pass time, randomness and I/O in

Code that computes a result takes the clock, the seed and the reader as arguments; it never reads `SystemTime::now`, `Instant::now` or an unseeded RNG itself (`clippy::disallowed_methods`).
Why: the same inputs then give the same output, so tests reproduce every decision.
Check: lint

### BP-RUST-076: Order iteration deterministically

Use `BTreeMap`/`BTreeSet` or sort before iterating when the order reaches output, hashes or logs; `HashMap` order is not stable.
Why: identical input then gives byte-identical output.
Check: review

## Hot paths

### BP-RUST-077: Name the hot paths

The governing spec lists which paths are hot (per frame, per item in a large batch, per request at load); setup, loading, tools and tests are not hot, and clear code wins there.
Why: performance rules apply where they pay, not everywhere.
Check: review

### BP-RUST-078: Allocate nothing in a hot path

A hot path makes no allocation in any form: `clone`, `to_vec`, `to_string`, `format!`, `collect`, `vec![]`, `Box::new`, or a small vector that spills.
Why: allocation dominates the cost of tight loops; moving it into a helper does not remove it.
Check: test

### BP-RUST-079: Size storage at setup and reuse it

Allocate hot-path buffers once at setup, then `clear()` and refill them.
Why: steady state then allocates nothing.
Check: review

### BP-RUST-080: Count allocations on each hot path

Give each new hot path a test that counts its allocations and fails when the count rises.
Why: the count is the check; the absence of the word `clone` is not.
Check: test

### BP-RUST-081: Index dense ids with a Vec

Use a `Vec` indexed by a dense id; keep maps for sparse keys.
Why: indexing is cheaper and more cache-friendly than hashing.
Check: review

### BP-RUST-082: Document in-place mutation on a hot path

When a measured hot path mutates or reuses a buffer that callers can observe, its doc comment states the invariant (aliasing, overwritten next call).
Why: callers can reason about aliases only when the rule is written.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (Rust state, buffers and hot paths)
