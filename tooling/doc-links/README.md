# doc-links

Checks links between code, tests and `docs/`, verifies doc kinds and tool registration, and gates commits on coverage drift.

## Run it

```sh
bin/doc-links check
```

The wrapper builds the Rust binary on first run and reuses it while sources stay unchanged.

## Examples

```sh
bin/doc-links check docs/repo          # check one scope
bin/doc-links refs docs/repo/hooks.md  # list citers and covered paths
bin/doc-links kinds                    # list every doc kind and its rules
```

## Reference

- Command reference and options: `docs/tooling/doc-links.tool.md`
- Design and citation forms: `docs/repo/doc-links.md`
- Doc kinds: `docs/repo/doc-kinds.md`
