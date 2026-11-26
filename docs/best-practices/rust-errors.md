---
description: How Rust code returns, propagates, maps and reports errors, and how it parses untrusted input.
tags: [best-practices, rust, errors]
---

# Rust errors and untrusted input

Expected failures are values. Ids continue the `BP-RUST` series from [[best-practices/rust]]. Where an error becomes
an exit code, a status or a log line is set here; the CLI contract is [[best-practices/tooling-cli]] and the event
levels are [[best-practices/logging]].

## Returning errors

### BP-RUST-044: Return typed errors, never panic

Return `Result<T, E>` for every failure a caller can meet; production code has no `unwrap`, `expect`, `panic!`, `unreachable!` or unchecked `get().unwrap()` (`clippy::unwrap_used`, `expect_used`, `panic`, `unreachable`, `get_unwrap`, `panic_in_result_fn`, `unwrap_in_result`).
Why: a panic ends the process without giving the caller a chance to recover; tests may still `unwrap`.
Check: lint

### BP-RUST-045: Define the error beside the operation

Declare one `thiserror` enum per module or operation, in the file of the operation that returns it, not in a shared errors module.
Why: the error changes with the code that produces it.
Check: review

### BP-RUST-046: One variant per caller decision

Give each failure the caller handles differently its own variant (`NotFound`, `Unsupported(u8)`); never collapse them into `String` or `anyhow` inside a library.
Why: callers match on variants, not on message text.
Check: review

### BP-RUST-047: Keep the cause chain

Wrap the underlying error with `#[source]` or `#[from]` instead of formatting it into the message.
Why: logs and reports walk the chain to the real cause.
Check: review

### BP-RUST-048: Propagate unchanged with ?

Pass a lower layer's error up with `?`; never swallow it, turn it into `None` or a default, or stringify it on the way.
Why: the outer boundary is the only layer that knows what the failure means to the user.
Check: review

### BP-RUST-049: Separate absence from failure

Return `Result<Option<T>, E>` when a lookup can legitimately find nothing: `Ok(None)` is a genuine miss, `Err` is a failed read.
Why: a failed read reported as "not found" gets cached or shown as missing content.
Check: review

### BP-RUST-050: Separate incomplete from unsupported input

A decoder distinguishes "more bytes needed" from "complete but unsupported", so a caller requests more data only in the first case.
Why: a stream reader otherwise waits forever or drops valid data.
Check: review

```rust
/// Identifies the reason a header cannot be read.
#[derive(Debug, PartialEq, Eq)]
enum HeaderError {
    /// The version byte has not arrived.
    Incomplete,
    /// The input declares an unknown format version.
    Unsupported(u8),
}

/// Checks the version and lends the remaining bytes to the decoder.
fn payload(input: &[u8]) -> Result<&[u8], HeaderError> {
    match input.split_first() {
        Some((&1, rest)) => Ok(rest),
        Some((&version, _)) => Err(HeaderError::Unsupported(version)),
        None => Err(HeaderError::Incomplete),
    }
}
```

### BP-RUST-051: Handle or explicitly discard every result

Use every returned value; discard one only with `let _ = ...;` and a comment saying why (`unused_results`, `#[must_use]`).
Why: an ignored `Result` is an ignored failure.
Check: lint

### BP-RUST-052: Compare errors by variant or code

Branch on error variants or stable codes, never on `to_string()` or message text.
Why: messages are reworded; variants and codes are the contract.
Check: review

## At the boundary

### BP-RUST-053: Map errors once, at the outer boundary

Convert domain errors to an exit code, HTTP status, field error or UI state in one place: the command, handler or screen that called the service.
Why: lower layers stay reusable from every entry point.
Check: review

### BP-RUST-054: Convert foreign failures in one place

The adapter that talks to an external system is the only code that turns its exceptions, status codes or error payloads into the domain error type.
Why: one place knows the foreign failure model; callers see one error type.
Check: review

### BP-RUST-055: Report an unexpected failure once

Log or report an unexpected failure where its final impact is known, not at every layer it passes through ([[best-practices/logging]]).
Why: one failure, one report.
Check: review

### BP-RUST-056: Give the caller a safe remedy

An error message says what failed and what to do next, and never includes a secret (a URL with its password, a token, a payload).
Why: the message reaches logs, terminals and agents.
Check: review

### BP-RUST-057: Return ExitCode from main

`main` returns `ExitCode` or a `Result` mapped to one; nothing calls `std::process::exit` (`clippy::exit`).
Why: destructors and buffered writers run, and the exit code follows [[best-practices/tooling-cli]].
Check: lint

### BP-RUST-058: Give wire errors a stable code

An error that crosses a process, network or language boundary carries a stable string code; the receiver branches on that code, never on type identity.
Why: type identity does not survive serialisation.
Check: review

## Untrusted input

### BP-RUST-059: Parse into validated types at entry

Turn external bytes, strings, paths, ids and settings into validated domain types where they enter the program, then pass only those types on.
Why: every later function can trust its inputs.
Check: review

### BP-RUST-060: Read untrusted bytes with checked accessors

Use `get`, `split_first`, `split_at_checked` and `try_into`; never index or slice with an offset that came from input (`clippy::indexing_slicing`, `string_slice`).
Why: a bad offset becomes an error instead of a panic.
Check: lint

### BP-RUST-061: Use checked arithmetic on input-derived numbers

Compute sizes and offsets from input with `checked_*`; document any intentional `wrapping_*` or `saturating_*` (`clippy::arithmetic_side_effects`).
Why: overflow on crafted input is a classic memory and logic bug.
Check: lint

### BP-RUST-062: Check the whole format

Check lengths, encoding, ranges and trailing data the format requires, with a test for each rejection.
Why: a parser that accepts trailing garbage accepts the next attack too.
Check: test

### BP-RUST-063: Drop unknown values and log them

When external data holds a value the domain does not know (a new kind, a removed option), drop or default it, log it at `debug` or `warn`, and keep going.
Why: one unknown value fails closed for that item instead of failing the whole read.
Check: test

## Covers

- `apps/`, `packages/`, `tooling/` (Rust error types, parsers and entry points)

## References

- [Codebase design](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/SKILL.md)
