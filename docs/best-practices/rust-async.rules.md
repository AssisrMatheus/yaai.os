---
description: How Rust code uses async, tasks, threads and shared state so concurrent work stays bounded, cancellable and deterministic.
tags: [best-practices, rust, concurrency]
---

# Rust async and concurrency

Ids continue the `BP-RUST` series. Speed claims follow [[best-practices/performance.rules]].

## Async

### BP-RUST-083: Use async for waiting, not for computing

Use `async` to wait on I/O; run blocking or CPU-heavy work off the executor (`spawn_blocking` or a worker pool).
Why: blocking an executor thread stalls every task on it.
Check: review

### BP-RUST-084: Await dependent work in order, start independent work together

Await an operation that needs a previous result after it; start independent operations together (`join!`, `try_join!`).
Why: sequential awaits on unrelated work add their latencies.
Check: review

### BP-RUST-085: Bound concurrency with a documented limit

Fan out with an explicit limit (`buffer_unordered(n)`, a semaphore) named as a constant and sized for the dependency.
Why: an unbounded fan-out overloads the source and the process.
Check: review

### BP-RUST-086: Drop lock guards before await

Never hold a mutex or `RefCell` guard across `.await` (`clippy::await_holding_lock`, `await_holding_refcell_ref`).
Why: other tasks block or deadlock while the guard waits.
Check: lint

### BP-RUST-087: Make async functions await

A function that never awaits is not `async` (`clippy::unused_async`).
Why: needless `async` hides that the work is synchronous and adds a state machine.
Check: lint

### BP-RUST-088: Give every task an owner

Every spawned task has an owner that keeps its handle, a cancellation path, and joins or aborts it on success and on error.
Why: orphan tasks leak work and hide their failures.
Check: review

### BP-RUST-089: Put a deadline on every external call

Wrap each network, IPC or subprocess call in a timeout named as a constant.
Why: a hung dependency otherwise hangs the caller forever.
Check: review

### BP-RUST-090: Retry only transient, idempotent work

Retry transient failures of idempotent operations with bounded, jittered backoff; retry a write only with a deduplication or transaction contract.
Why: retrying a non-idempotent write duplicates it; unbounded retries become a storm.
Check: review

### BP-RUST-091: Document cancellation safety

Futures used in `select!` or with timeouts document whether they are cancel-safe; state that would be lost on cancellation is moved out first.
Why: a dropped future silently abandons partial work.
Check: review

### BP-RUST-092: Discard stale results

A response for an older request never overwrites state set by a newer one: compare a request token or generation before applying it.
Why: out-of-order completion otherwise shows the wrong data.
Check: test

## Threads

### BP-RUST-093: Run independent CPU work in parallel

Split CPU work whose parts are independent across threads when the result cannot depend on which finishes first.
Why: batch tools, importers and validators scale with cores.
Check: review

### BP-RUST-094: Use a pool that lives as long as the job

Create one pool per job sized from `std::thread::available_parallelism()`, never a thread per item; use `std::thread::scope` to lend borrowed data.
Why: thread start-up costs more than small work items.
Check: review

### BP-RUST-095: Shut workers down on every path

Close the work channel and join every handle on success and on error.
Why: a worker left running holds resources and swallows its panic.
Check: review

### BP-RUST-096: Divide data instead of sharing it

Give each thread its own part (`chunks_mut`, `split_at_mut`, one output slot per input); keep `Arc<Mutex<_>>` for rare coordination, never inside the hot loop.
Why: a lock in the loop turns many threads back into one.
Check: review

### BP-RUST-097: Merge results in input order

Reassemble outputs, logs and hashes in input order, so one thread and sixteen produce identical bytes.
Why: completion order varies per run.
Check: test

### BP-RUST-098: Pick failures deterministically

When several parts fail, report the first failure in input order, not the first to complete.
Why: the same input then gives the same error.
Check: test

### BP-RUST-099: Measure one thread against many

Record the one-thread and many-thread timings and keep one thread when the job is too small to win.
Why: threads add hand-off and contention costs.
Check: review

### BP-RUST-100: Justify a thread-pool crate by measurement

Use `std::thread` for a scoped fan-out; add a pool crate only when work stealing or nested parallelism is measured to pay ([[best-practices/dependencies.rules]]).
Why: a pool crate is a dependency like any other.
Check: review

## Shared state

### BP-RUST-101: Keep mutable state out of globals

Hold mutable state in owned values passed to the code that needs it; no `static` mutable state, thread-locals or lazily initialised mutable singletons.
Why: global state leaks between requests, users and tests.
Check: review

### BP-RUST-102: Tie subscriptions to their owner

Listeners, timers and subscriptions are released when their owner drops (a guard with `Drop`), never left registered.
Why: leaked subscriptions keep firing after their owner is gone.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (async code, tasks and threads)
