---
description: Select one shard of a test plan using recorded file durations.
tags: [tooling, ci, rust]
---

# test-shard

## Synopsis

`bin/test-shard --timings .github/ci/timings.json --shard 1 --count 2 [--json]`

## Commands

Read eligible test file IDs from stdin, one per line. Assign descending durations to the lightest shard, breaking ties by file ID and then shard number. Weight unrecorded files by the median of the timing snapshot (1 ms for an empty snapshot). Empty shards succeed.

## Options

`--timings` takes Bun's version 1 JSON file of integer milliseconds. Both `--shard` and `--count` start at one. Use `--json` for the complete selection. See [[repo/ci]] for discovery and measurement.

## Exit codes

0 for success or help, 2 for invalid inputs, 1 for a read or output failure.

## Output

Plain output reports the shard and file count. JSON contains a single object with a `files` array in assignment order.

Across 20 warm invocations, wrapper startup plus help took 41 ms median and 42 ms p95. An empty selection took 38 ms median and 38 ms p95.

## Covers

- `tooling/test-shard/`
- `bin/test-shard`
