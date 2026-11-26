# Test shards

Find which tests belong on a runner. From the repository root, try:

```sh
jq -r '.files | keys[]' .github/ci/timings.json |
  bin/test-shard --timings .github/ci/timings.json --shard 1 --count 2 --json
```

The [command reference](../../docs/tooling/test-shard.tool.md) explains how new tests get a weight.
