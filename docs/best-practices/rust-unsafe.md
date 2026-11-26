---
description: When unsafe Rust is allowed, how each unsafe block is proved and tested, and how values cross an FFI boundary.
tags: [best-practices, rust, unsafe]
---

# Rust unsafe and FFI

Safe Rust is the default. Ids continue the `BP-RUST` series.

## Unsafe code

### BP-RUST-103: Deny unsafe by default

`unsafe_code` is denied for the workspace; a crate that needs none never opts in.
Why: most crates never need it, so the compiler enforces that.
Check: lint

### BP-RUST-104: Opt in item by item

Allow `unsafe` only at an FFI boundary module or on a measured hot path, with `#[expect(unsafe_code, reason = "...")]` on the smallest item.
Why: every exception is visible, local and justified.
Check: lint

### BP-RUST-105: Wrap unsafe behind a safe function

Put the unsafe operation in the smallest function that needs it, behind a safe signature; that function itself establishes every requirement its safety comment names.
Why: callers cannot break an invariant they never see.
Check: review

### BP-RUST-106: Prove every unsafe block

Write `// SAFETY:` above each `unsafe` block, proving the bounds, alignment, initialisation, lifetime, aliasing and threading requirements that apply (`clippy::undocumented_unsafe_blocks`).
Why: the proof is what a reviewer checks.
Check: lint

### BP-RUST-107: One unsafe operation per block

Each block holds one unsafe operation, also inside an `unsafe fn` (`clippy::multiple_unsafe_ops_per_block`, `unsafe_op_in_unsafe_fn`).
Why: each operation gets its own proof.
Check: lint

### BP-RUST-108: Document the contract of unsafe interfaces

Every `unsafe fn` and `unsafe trait` has a `# Safety` section listing the caller's obligations (`clippy::missing_safety_doc`).
Why: callers must know what they promise.
Check: lint

### BP-RUST-109: Validate input before it reaches unsafe code

Validate offsets and lengths from files, the network or another process with checked reads before they reach unsafe code.
Why: unchecked input plus unsafe is a memory-safety hole.
Check: review

### BP-RUST-110: Justify unsafe by a safety argument, not a borrow error

Unsafe that only silences a borrow or lifetime error is rejected ([[best-practices/rust-ownership]]).
Why: a borrow error alone supplies no safety argument.
Check: review

### BP-RUST-111: Write and measure the safe form first

Unsafe on a hot path replaces a safe form that was written and measured first; the `reason` of the `expect` carries the figure and where it is recorded.
Why: better layouts and hoisted checks often reach the same speed safely.
Check: review

### BP-RUST-112: Compare unsafe code with a safe reference

Keep the safe implementation under `#[cfg(test)]` and compare outputs on empty, single, last-valid and maximum-size inputs plus a seeded sweep, on every supported target.
Why: the comparison proves the optimisation changed no result.
Check: test

### BP-RUST-113: Remove unsafe that fails a test

A failure of any unsafe parity or sanitizer test removes the unsafe code, whatever it saved.
Why: undefined behaviour is a wrong result.
Check: review

### BP-RUST-114: Run Miri or sanitizers on unsafe code

Exercise unsafe code under Miri or a sanitizer where the target supports it.
Why: they catch undefined behaviour tests do not observe.
Check: test

## FFI

### BP-RUST-115: Validate before forming views

At an FFI boundary, check lengths, null pointers and ownership before creating slices or references from foreign data.
Why: a slice over invalid memory is undefined behaviour at once.
Check: review

### BP-RUST-116: Specify the memory contract

For every value crossing FFI, document its representation (`#[repr(C)]`), allocator, release function and how long it stays valid.
Why: both sides must free memory the same way exactly once.
Check: review

### BP-RUST-117: Keep unwinding out of foreign frames

A Rust panic never unwinds into foreign code: catch it at the boundary or build with `panic = "abort"`, and use `extern "C-unwind"` only by decision.
Why: unwinding across an FFI frame is undefined behaviour.
Check: review

### BP-RUST-118: Keep crates target-neutral

Only the boundary module knows about the foreign interface; the rest of the crate builds for every supported target.
Why: logic stays testable natively.
Check: review

### BP-RUST-119: Export a narrow C interface

Exports are `extern "C"` functions with scalar arguments, buffers as pointer plus length, and symbol names carrying a crate prefix.
Why: the foreign side decodes plain values, and symbols cannot collide.
Check: review

### BP-RUST-120: Reuse buffers that cross every frame

Allocate buffers that cross the boundary per frame or per call once, and reuse them; the other side reads them in place.
Why: per-call allocation and copying dominates boundary cost.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (unsafe code and FFI modules)
