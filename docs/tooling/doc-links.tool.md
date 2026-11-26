---
description: Command reference, options, exit codes and sample output for the doc-links CLI.
tags: [tooling, docs, rust]
---

# doc-links

Checks links between code, tests and `docs/`, verifies doc kinds and tool registration, gates commits on coverage
drift, and injects checked edges into graphify. Design and citation forms live in [[repo/doc-links]]; doc kinds live
in [[repo/doc-kinds]]; CLI output rules live in [[best-practices/tooling-cli.rules]].

## Synopsis

```sh
bin/doc-links [options] <command>
pnpm --filter @yaai/doc-links start -- [options] <command>
```

`bin/doc-links` runs an up-to-date release or debug binary under `target/`, builds the release binary if sources
changed, or prints one notice to stderr and exits 0 when `cargo` is absent.

## Commands

| Command                 | Does                                                                                                                                                               |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `check [paths...]`      | Validate vault frontmatter, wikilinks, indexes, citations, `Covers`, doc kinds and tool registration; filter reported findings to repo-relative `paths` when given |
| `refs <query>`          | List outgoing and incoming `cites`, `validates` and `covers` edges for a repo path, doc anchor, invariant ID or quoted `ADR NNNN`                                  |
| `matrix`                | List invariant IDs and governing docs with `Covers` that have no test citation                                                                                     |
| `graph`                 | Replace `doc-links` edges in `graphify-out/graph.json` atomically; prints a skip row when the graph is absent                                                      |
| `kinds`                 | Print every doc kind from `tooling/doc-links/kinds.toml` with its file pattern, folders and requirements                                                           |
| `commit <message-file>` | Check staged files in the Git index and block covered code changes whose governing doc is unstaged unless waived by a non-empty `Spec-unchanged:` trailer          |

## Options

| Argument / option | Default                                            | Meaning                                                                                                                       |
| ----------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `--root <dir>`    | nearest enclosing Git checkout (or `docs/` parent) | Repository root to inspect                                                                                                    |
| `--json`          | off                                                | Emit one JSON result object on stdout (`command`, `status`, `total`, `shown`, `findings`, `rows`); findings also go to stderr |
| `-v`              | off                                                | Emit structured JSON diagnostic logs on stderr (also controlled by `DOC_LINKS_LOG`, e.g. `doc-links.scan:debug`)              |
| `--limit <n>`     | `50`                                               | Maximum findings and rows displayed; summary counts still report the full total                                               |
| `-h`, `--help`    | off                                                | Print command help with examples and exit 0                                                                                   |

## Exit codes

- `0`: Operation succeeded cleanly (`check` or `commit` had zero findings; `refs`, `matrix`, `kinds` or `graph` completed or skipped cleanly; or `bin/doc-links` skipped because `cargo` is absent).
- `2`: `check` or `commit` found repairs the caller must make. Each finding goes to stderr as `<file>:<line>: <code> <remedy>`, followed by the summary line.
- `1`: Invalid CLI arguments, malformed `kinds.toml`, or operational I/O failure (`E_RUN` on stderr; nothing on stdout).

## Output

```text
check ok findings=0 rows=0 shown=0
```

## Covers

- `tooling/doc-links/`
- `bin/doc-links`
