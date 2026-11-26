---
description: Which events to log at which level, which fields they carry, how records are filtered, rendered and redacted, and when a failure is reported.
tags: [best-practices, logging]
---

# Logging

The logging standard of INV-LOGGING-001: apps, packages and tools log extensively, in one line format shared by Rust
and TypeScript. Rust uses `tracing`; TypeScript uses one scoped logger with the same rendering.

## Events

### BP-LOG-001: Log through a scoped logger, never print

Emit diagnostics through the structured logger; never `println!`, `eprintln!`, `dbg!` or `console.log` for diagnostics (`clippy::print_stdout`, `print_stderr`, `dbg_macro`, oxlint `no-console`). A tool's result output goes through its writer ([[best-practices/tooling-cli.rules]]).
Why: unscoped prints cannot be filtered, redacted or routed.
Check: lint

### BP-LOG-002: Let the executable own the sink

Libraries emit events and return errors; only the executable installs the subscriber, the filter and the sink.
Why: a caller can then enable one subsystem without changing a dependency.
Check: review

### BP-LOG-003: Give every record the standard fields

Each record has a UTC timestamp with milliseconds, a level, a stable dot-separated scope (`catalog.read`) and a short title; varying facts go in named fields.
Why: filters, sorting and aggregation work the same across runs and runtimes.
Check: test

### BP-LOG-004: Keep ids and values in fields

Put ids, counts and durations in named fields, never interpolated into the title.
Why: titles stay constant, so they group and search.
Check: review

### BP-LOG-005: Use the lowest level that matches the outcome

Pick the level from this table.
Why: a consistent level makes the default filter show exactly what needs attention.
Check: review

| Level   | Use for                                                                                  | Fields                                                        |
| ------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `debug` | adapter and cache decisions, query names, hit or miss, retry attempts, state transitions | duration, count where useful                                  |
| `info`  | process ready or stopped, a meaningful operation completed                               | operation, outcome, elapsed time                              |
| `warn`  | recovered failure, degraded mode, discarded input, retries exhausted with a fallback     | reason, recovery                                              |
| `error` | unexpected terminal failure at its handling boundary                                     | stable code, operation, safe cause, correlation id, next step |

### BP-LOG-006: Summarise a retried success once

An operation that retried and then succeeded logs one final success; the attempts are `debug`.
Why: recovered noise must not look like failure.
Check: review

### BP-LOG-007: Open a span at every entry point

A command, request or job opens a span carrying the operation and a correlation id, propagated across async work.
Why: every event in the operation can be found by one id.
Check: review

### BP-LOG-008: Log completion at the edge and timings at the adapter

Record an operation's completion and elapsed time at the outer boundary, and each dependency call's timing in its adapter.
Why: slow operations can be traced to the slow dependency.
Check: review

### BP-LOG-009: Record writes and rejected writes

Log every mutation and rejected write with its outcome and a safe object id.
Why: effects can be traced after the fact.
Check: review

### BP-LOG-010: Instrument each transport once

Log calls to each transport (database, RPC, HTTP, IPC) from one middleware in its client, scoped by operation kind.
Why: every call is covered without call-site logging.
Check: review

```rust
tracing::info!(target: "catalog.read", count, elapsed_ms, "read completed");
```

```ts
log.info('read completed', { count, elapsed_ms });
```

## Reporting failures

### BP-LOG-011: Report only failures the user sees

Raise an error report (an alerting event) only when a user-visible operation ultimately failed and nothing above will report it; a recovered failure is a `warn` or `info` log with ids.
Why: over-reporting buries real regressions; under-reporting hides broken screens.
Check: review

### BP-LOG-012: Report a returned error value once, where it was produced

A caller that receives an error value from a service logs it if it needs a trail but does not report it again; the service that produced it already decided.
Why: one failure, one report.
Check: review

### BP-LOG-013: Return validation remedies instead of logging errors

Expected validation failures return a field or input remedy and are not logged as errors.
Why: user mistakes are not system failures.
Check: review

### BP-LOG-014: Report at the call site, not inside transports

Never hand an error-reporting function to a transport client; report where the call's meaning is known.
Why: a transport-level report fires first with less context and wins deduplication.
Check: review

### BP-LOG-015: Give every failing unit a reporting boundary

Every screen region, command or job that can fail sits under a boundary that reports what it catches.
Why: a failure caught by a non-reporting boundary disappears.
Check: review

### BP-LOG-016: Keep noise filters in one module

Put report filters in one module; each entry names what produces the noise and why it cannot reach a user.
Why: an undocumented filter can never be safely removed.
Check: review

### BP-LOG-017: Filter narrowly on the whole cause chain

Match a filter on an anchored token or the error's shape across its full cause chain; never broad patterns (`/failed/i`) or a single message.
Why: broad patterns swallow real regressions.
Check: review

### BP-LOG-018: Always report actionable failures

Connection refused, unknown host and similar failures are always reported.
Why: they mean a wrong configuration or an outage.
Check: review

### BP-LOG-019: Wire filters on every runtime

A filter applies on every runtime that can produce the event (core, UI, worker).
Why: a half-wired filter leaves half the noise.
Check: review

### BP-LOG-020: Group ambiguous failures instead of filtering them

When unsure whether a failure matters, give it a grouping key (fingerprint) rather than dropping it.
Why: grouping separates noise without deciding something you may be wrong about.
Check: review

### BP-LOG-021: Classify by type before code

Check an error's type name before reading a common field such as `code`; prefer an allowlist of expected errors to a denylist.
Why: an unrelated error with the same `code` string would be silenced.
Check: review

### BP-LOG-022: Keep the release id identical

The release or version id is the same in the runtime's reporting setup and in the uploaded debug symbols; symbols are uploaded, never served publicly.
Why: a mismatch leaves every stack unreadable.
Check: review

### BP-LOG-023: Keep telemetry endpoints out of routing and tracing

Endpoints that receive telemetry are excluded from locale redirects and from tracing.
Why: a redirect drops the events, and tracing them is a feedback loop.
Check: review

## Filtering and rendering

### BP-LOG-024: Filter with one grammar and one variable

Default to the `error` floor. Accept a comma-separated filter of global levels, `*` for everything and `scope[:level]` for a scope and its dot-children (`warn,catalog.read:debug`); tokens are trimmed and lowercased; with several floors the lowest wins. One variable name configures every runtime.
Why: one filter syntax across languages and tools.
Check: test

### BP-LOG-025: Check enablement before building expensive fields

Build costly field values only when the record's level and scope are enabled.
Why: disabled logging must cost nothing.
Check: review

### BP-LOG-026: Send diagnostics to stderr

Terminal diagnostics at every level go to stderr; stdout carries only the operation's result. A browser sink uses the console method matching the level.
Why: results stay machine-readable.
Check: test

### BP-LOG-027: Colour only a terminal, only the head

Colour the timestamp, level and scope only when the stream is a TTY and `NO_COLOR` is unset; child scopes share their parent's colour (hash of the first segment). Plain and structured modes carry every fact without styling.
Why: pipes and files stay clean, and related scopes stay recognisable.
Check: test

### BP-LOG-028: Render one event on one line

Render a record as one line: ISO 8601 UTC timestamp with milliseconds, lowercase level, scope, title, then fields with sorted keys. Escape control characters, normalise scope characters outside `[a-z0-9_.-]` to `_`.
Why: one record per line keeps output greppable and unforgeable.
Check: test

```text
2026-11-26T12:00:00.000Z [info] [catalog.read] read completed count=12 elapsed_ms=8
```

### BP-LOG-029: Bound every value

Bound strings and collections, cap container depth at four, mark cycles, print byte buffers as their length, collapse repetition to counts and omit stack traces unless asked.
Why: one field must not flood a terminal or an agent's context.
Check: test

### BP-LOG-030: Render numbers and dates exactly

Render finite numbers with round-trip precision, normalise negative zero, write non-finite values explicitly (as strings in JSON), dates in UTC and errors as their safe type and message.
Why: the same value renders the same way in every runtime.
Check: test

### BP-LOG-031: Redact before any sink

Redact before rendering or serialising: values under sensitive keys (password, secret, token, API key, authorization, cookie, credential, private key) at any depth, URL fragments carrying `=`, and `key=value` text whose key is sensitive.
Why: redaction is the safety net when a secret reaches a log call.
Check: test

### BP-LOG-032: Attach ids, not payloads

Log and report object ids, never personal data, request bodies, auth headers or URLs carrying tokens.
Why: logs travel further than the data's owner intended.
Check: review

### BP-LOG-033: Test rendering with a shared fixture

Rust and TypeScript render a shared fixture byte for byte with a fixed clock: levels, scope matching, formatting, redaction, truncation, JSON validity and pipe output, including a secret nested in an error and a newline in a title.
Why: both runtimes then emit the same logical event identically.
Check: test

## Diagnostics for agents

### BP-LOG-034: Make a diagnostic actionable

A development diagnostic carries a stable code, the source location and the fix, plus a link to the human doc and to the agent repair section anchored by the code.
Why: an agent can repair the mistake without guessing.
Check: review

### BP-LOG-035: Use one code everywhere

The same diagnostic code appears in human text, structured output and the repair guide.
Why: an agent maps output to its fix by the code alone.
Check: review

### BP-LOG-036: Warn about structural problems in development

Development builds warn about structural problems they can detect: request waterfalls, effect loops, wide fan-out and work pending too long since the last input.
Why: these never fail a test but slow every run.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (logger setup, events, reporters and sinks)

## References

- [Agent-readable diagnostics](https://www.solidjs.com/blog/solid-2-0-the-long-hold/)
- [Compact command output](https://raw.githubusercontent.com/rtk-ai/rtk/develop/README.md)
