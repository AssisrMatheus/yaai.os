---
description: How a repository tool is invoked, what it prints on stdout and stderr, which exit code it returns, and how it stays fast.
tags: [best-practices, tooling]
---

# Tooling CLI

The contract of INV-TOOLING-001 and INV-TOOLING-002 (ADR 0009). Tools are read by agents first, people second, and
their output passes through rtk ([[repo/rtk]]).

## Invocation

### BP-CLI-001: Turn repeated commands into tools

Write a tool under `tooling/` when a command will run more than once or its flags, paths or parsing would need rediscovering; a one-off inspection stays a direct command.
Why: the next agent finds the tool instead of reinventing it.
Check: review

### BP-CLI-002: Extend a tool family before adding a tool

Add a subcommand to an existing tool before creating a near-duplicate tool.
Why: one place per kind of job.
Check: review

### BP-CLI-003: Register the tool where agents look

A new tool gets a `bin/<name>` wrapper that only locates and runs it, and its governing doc lists it in `## Covers` in the same change ([[repo/structure]]).
Why: a tool nobody can find does not exist for the next agent.
Check: lint

### BP-CLI-004: Take every input non-interactively

Every input comes from arguments, flags, files or stdin; a tool never waits for interactive input.
Why: agents and CI cannot answer prompts.
Check: test

### BP-CLI-005: Share one operation between CLI and any other front end

When a tool also has a page or app, both call the same operation.
Why: the CLI stays the reproducible way to run it.
Check: review

### BP-CLI-006: Answer --help without side effects

`--help` works at the root and on every subcommand without configuration, network or a project scan; it prints the synopsis, purpose, inputs, defaults, output, exit codes and one runnable example, and exits 0.
Why: help is discoverable without reading the source.
Check: test

### BP-CLI-007: Document every entry point

Every command and subcommand has a doc comment stating what it does.
Why: graphify indexes it, so `graphify query` finds the tool by purpose.
Check: lint

### BP-CLI-008: Offer --json for results

A result-producing command accepts `--json` and then prints exactly one valid JSON document on stdout, including on empty success, with stable, documented keys checked by a parser test.
Why: consumers never scrape display text.
Check: test

## Output

### BP-CLI-009: Lead with the verdict

Plain stdout opens with the command and subject, then the verdict and counts.
Why: the answer comes before the detail.
Check: test

### BP-CLI-010: Group evidence by kind

Group results under short headings by kind, use rows or tables for records, and print repository-relative paths.
Why: grouped evidence is easy to scan and quote.
Check: review

### BP-CLI-011: Keep output deterministic

Identical input gives identical order and wording.
Why: output can be diffed, cached and used as a test oracle.
Check: test

### BP-CLI-012: Print a pass as one line

A passing check prints one summary line; failures print with their evidence and passing items collapse into a count.
Why: an agent reads the failures, not a wall of passes.
Check: review

### BP-CLI-013: Bound output and say so

Limit output by default, report shown and total counts, and say how to see the rest (`--limit`, a file).
Why: a silent cut looks like the whole answer.
Check: test

### BP-CLI-014: Send large results to a file

A result too large for a terminal goes to a file; stdout prints its path and a summary.
Why: stdout stays small while the full data is kept.
Check: review

### BP-CLI-015: Print the evidence a claim needs

Include the path, line, rule id, offset or hash that identifies the repair or lets the claim be cited.
Why: results can be acted on without re-running.
Check: review

### BP-CLI-016: Keep verdict lines stable

The verdict, each failure and the summary sit on lines of their own, in the same words every run.
Why: rtk's filters keep stable lines and drop decoration, and the verdict survives line filtering.
Check: review

### BP-CLI-017: Keep stdout for the result

Logs, progress and warnings go to stderr; stdout carries only the result ([[best-practices/logging.rules]]).
Why: callers capture the result cleanly.
Check: test

### BP-CLI-018: Decorate nothing outside a terminal

Disable colour, spinners, progress bars, cursor control, emoji and box drawing when the stream is not a TTY; stdout is never decorated.
Why: decoration costs tokens and carries no information.
Check: test

### BP-CLI-019: Let a required recovery message through every filter

A message the caller needs to recover is printed regardless of the log filter.
Why: a quiet run must still say what to do.
Check: review

### BP-CLI-020: Make each failure actionable

A failure diagnostic carries a stable code, the location, the observed condition and the next command or edit, plus a spec link when it explains the rule; it shows the smallest input that identifies the problem.
Why: the agent can recover without guessing.
Check: review

### BP-CLI-021: Pass raw output through when a filter fails

A tool that filters or transforms another command's output prints the raw output and passes the original exit code through when its own processing fails.
Why: a filter must never hide the result it was meant to shorten.
Check: test

## Exit codes

Choose by who can resolve the outcome. A shell or CI caller checks the exact code; the "non-blocking" meaning of 1
comes from hook handling ([[repo/hooks]]). A failed tool produced no verdict.

| Exit | Decision | Example stdout | Example stderr |
| --- | --- | --- | --- |
| 0 | Operation completed and the contract holds | `ok checked=18` | Empty by default |
| 2 | Caller must change input or act before proceeding | `blocked invalid=1` for a completed validation report | `E_LINK docs/example.md:8 unknown target. Update the link and rerun the check.` |
| 1 | The tool or its dependency failed, so the result is unavailable | Empty | `E_INDEX index read failed. Retry with --rebuild-index.` if that option exists |

### BP-CLI-022: Exit 0 only when the contract holds

Return 0 only when the operation completed and found nothing the caller must act on.
Why: callers proceed on 0 without reading output.
Check: test

### BP-CLI-023: Exit 2 when the caller must act

Return 2 for invalid options, missing arguments and completed validations that found problems; stderr states what to change.
Why: hooks block on 2 and show stderr as the reason.
Check: test

### BP-CLI-024: Exit 1 when the tool itself failed

Return 1 when the tool or a dependency it relies on failed and no verdict exists (a crash, an unreadable own index, an unavailable service). A missing caller-supplied file is 2; a failure reading the tool's own index is 1.
Why: the caller retries or reports instead of fixing its input.
Check: test

### BP-CLI-025: Preserve the exit code through wrappers

Shell wrappers, pipes and subprocess adapters pass the tool's exit code through unchanged.
Why: a wrapper that turns 2 into 1 turns "fix your input" into "the tool broke".
Check: test

### BP-CLI-026: Test stdout, stderr and the exit code separately

For each of 0, 1 and 2, a test asserts stdout, stderr and the exit code on their own.
Why: each stream has its own contract.
Check: test

### BP-CLI-027: Keep JSON output consistent with the exit code

With `--json`, a completed validation may print its report and exit 2; when there is no report (exit 1), stdout stays empty.
Why: a consumer parses stdout only when a report exists.
Check: test

### BP-CLI-028: Print one JSON object or plain text to hooks

A tool called by an agent hook prints either exactly one JSON object with nothing around it, or plain text, on stdout.
Why: anything else is read as a parse failure.
Check: test

## Speed

### BP-CLI-029: Budget warm start-up

Keep warm start-up of `--help` and a no-work run at 100 ms each (initial target); measure the installed entry point over at least 20 warm runs and record the median and p95.
Why: tools run inside edit and stop hooks.
Check: test

### BP-CLI-030: Measure build time apart from work

Report build time, start-up and actual work separately when measuring.
Why: a regression must be attributable.
Check: review

### BP-CLI-031: Work incrementally

Discover changed inputs first, reuse parsed results, and bound parallel I/O.
Why: most runs touch a few files.
Check: review

### BP-CLI-032: Key persistent caches on everything that matters

Key a persistent cache on content, configuration, relevant environment and tool version; replace entries atomically after successful work; rebuild a corrupt entry.
Why: a cache must never return a result for different inputs.
Check: test

### BP-CLI-033: Register task-shaped work with Turborepo

Deterministic, task-shaped work runs as a Turborepo task with declared dependencies, inputs, environment and outputs; side effects and watch processes stay uncached ([[repo/turborepo/tasks]]).
Why: unchanged reruns take under a second.
Check: review

### BP-CLI-034: Check cold and warm runs agree

A test runs the tool cold and warm and compares results, and checks that changing an input invalidates the cache.
Why: a green cache hit must mean the same thing as a fresh run.
Check: test

## Covers

- `tooling/`
- `bin/`

## References

- [Hook exit code handling](https://code.claude.com/docs/en/hooks.md#exit-code-output)
- [Compact command output](https://raw.githubusercontent.com/rtk-ai/rtk/develop/README.md)
- [Diagnostics for agents](https://www.solidjs.com/blog/solid-2-0-the-long-hold/)
