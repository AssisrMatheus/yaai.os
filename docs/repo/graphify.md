---
description: What graphify does for agents here, how the graph is built and refreshed, how to query it, and how to fix it.
tags: [repo, graphify, tooling, agents]
---

# graphify: the code and doc graph

graphify turns the repository into a local knowledge graph (`graphify-out/graph.json`, AST mode: tree-sitter, no LLM,
nothing leaves the machine). Agents query it instead of grepping (ADR 0008). Docs are in the graph too: each page,
heading, wikilink and backticked code path is a node or an edge ([[repo/documentation]]); [[repo/doc-links]] adds the
citation and `## Covers` edges.

## Wiring

| Piece         | File                                                                                                      | Role                                                                                                        |
| ------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Ignore list   | `.graphifyignore`                                                                                         | Excludes `vendor/`, build and dependency output, `.agents/skills/graphify/`, `.old/` and `docs/.obsidian/`. |
| MCP server    | `.mcp.json`, `.codex/config.toml`                                                                         | Runs `graphify-mcp` on the graph found through the Git root, so it works from a subdirectory.               |
| Rebuild       | `.husky/post-commit`, `post-merge`, `post-checkout` (branch switches only) -> `.husky/graphify-update.sh` | Starts a detached `GRAPHIFY_BACKEND=none graphify update .` when graphify and a first graph exist.          |
| Search advice | `graphify hook-guard` from [[repo/hooks]]                                                                 | Reminds agents to query the graph before broad Grep or Read.                                                |
| Output        | `graphify-out/`                                                                                           | Git-ignored.                                                                                                |

Overlapping events leave a `graphify-out/.update-pending` marker, so one worker drains them. Hooks fail open: a missing
binary, a missing graph or a crash never blocks Git. The tested version is 0.9.111; the MCP extra is
`graphifyy[mcp]` ([[repo/platforms]]).

## One-time steps after cloning

1. Install graphify with the MCP extra ([[repo/platforms#First steps after cloning]]).
2. From the root run `GRAPHIFY_BACKEND=none graphify update .` once. The git hooks keep it fresh afterwards.

Never run `graphify <agent> install`: it writes user-level config, and the shell policy blocks it.

## How an agent uses it

Ask the graph before grepping or reading files.

```sh
graphify query "where are doc citations parsed?"   # plain-language question, scoped subgraph
graphify explain "<symbol or page>"                # one node and its edges
graphify path "<A>" "<B>"                          # shortest path between two nodes
GRAPHIFY_BACKEND=none graphify update .            # after uncommitted edits
```

The MCP tools (`query_graph`, `get_node`, `get_neighbors`, `shortest_path`, `god_nodes`, `graph_stats`) answer the
same questions in runtimes that load `.mcp.json`. The graph reflects the last commit; run `update` to include
uncommitted edits. Fall back to Grep for exact strings the graph does not hold.

## Check that it works

- `graphify-out/GRAPH_REPORT.md` exists.
- `graphify explain <an indexed symbol>` prints a node.
- The MCP `graph_stats` tool returns counts.

## Failure modes

| Symptom                                                      | Fix                                                                               |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| No `graph.json`                                              | Run the first build.                                                              |
| Graph stale after commits                                    | graphify missing from PATH in the hook environment, or the first build never ran. |
| `graphify-out/.update.lock` stays after a kill or power loss | Check that no worker runs, remove the empty directory, rerun the first build.     |
| A heavily pruned graph refuses to update                     | Rerun with graphify's explicit `--force`.                                         |

## Covers

- `.graphifyignore`
- `.mcp.json`
- `.husky/graphify-update.sh`
- `.husky/post-commit`
- `.husky/post-merge`
- `.husky/post-checkout`

## References

- [Graphify](https://github.com/Graphify-Labs/graphify)
