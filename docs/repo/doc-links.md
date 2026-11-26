---
description: The checked links between code, tests and docs, and the doc-links CLI that checks them, gates commits and adds them to the graph.
tags: [repo, docs, tooling]
---

# Links between code, tests and docs

> Status: being built in `tooling/doc-links`, run through `bin/doc-links`. Decision: ADR 0013.

Agents move through the repository by following references: from a function or a test to the invariant, spec, screen
or ADR it implements or checks, and back from a rule to the code and tests that carry it. Links written in a fixed
form and checked by a machine stay true; prose links drift. This page sets the forms. `doc-links` checks them, gates
commits on them, reports which rules tests pin, and adds them to the graphify graph (INV-DOCS-007).

## Nodes

| Node             | Lives in                                                                             | Identified by                                              |
| ---------------- | ------------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| Invariant        | `docs/invariants/<domain>.invariants.md`                                             | `INV-<DOMAIN>-NNN` (heading `## INV-<DOMAIN>-NNN: <rule>`) |
| ADR              | `docs/adr/NNNN-<slug>.adr.md`                                                        | `ADR NNNN`                                                 |
| Spec or doc page | `docs/<scope>/<page>.md`                                                             | `docs/<path>.md#<heading-slug>`, `## Covers`               |
| Screen doc       | `docs/screens/<name>.screen.md`                                                      | `docs/screens/<name>.screen.md`                            |
| Code             | `apps/`, `packages/`, `tooling/`, `bin/`                                             | path, `path::Symbol`                                       |
| Test             | `*_test.rs`, Rust `#[cfg(test)]` modules, `tests/`, `*.test.ts`, `*.spec.ts`, `e2e/` | path                                                       |

## Edges

| Edge              | Written as                                           | Checked by                        | Graph relation |
| ----------------- | ---------------------------------------------------- | --------------------------------- | -------------- |
| Code to rule      | A citation in the comment above the item             | `check`, commit gate              | `cites`        |
| Test to rule      | The same citation in the test's comment or name      | `check`, commit gate              | `validates`    |
| Doc to code       | A `## Covers` list of code paths                     | `check`, commit gate              | `covers`       |
| Doc mentions code | A code path in backticks; `path::Symbol` for an item | none (history may name old files) | `references`   |

## Citing docs from code

In the comment right above the item it governs (rustdoc `///` or `//!`, JSDoc, `#` in shell and TOML):

| Form                                    | Cites                                         |
| --------------------------------------- | --------------------------------------------- |
| `spec: docs/<path>.md#<heading-slug>`   | A doc page or one of its headings             |
| `screen: docs/screens/<name>.screen.md` | The screen or subview doc the code reproduces |
| `INV-<DOMAIN>-NNN`                      | An invariant                                  |
| `ADR NNNN`                              | A decision                                    |

```rust
/// spec: docs/repo/history.md#file-name
/// INV-DOCS-004
fn old_entry_name(name: &str, day: Date) -> String {
```

- The anchor is GitHub's slug of the heading: lower case, spaces to `-`, punctuation dropped, `-1`, `-2` for repeats.
- The citation attaches to the first line below the comment block that is not a comment, attribute or decorator, so
  the graph links that function, type or constant. A module-level citation goes in `//!` or the file's opening
  comment.
- Cite rules for the what. Rationale goes in a comment starting `WHY:` or `NOTE:` beside the code it explains.
- Inside a Rust `#[cfg(test)]` module, doc paths count only on comment lines: fixtures write made-up paths in strings.

## Tests that pin rules

A test that checks a rule cites it the same way: in the comment above it or in its name
(`fn inv_docs_004_old_entry_needs_commit()`, `test("INV-DOCS-004: old entry needs a commit", ...)`). The citation
becomes a `validates` edge. A rule that only code cites is implemented but not pinned; a change that adds or changes
such a rule adds the test that pins it.

## Listing covered code

A page that governs code lists it ([[repo/documentation#The Covers section]]):

```markdown
## Covers

- `tooling/doc-links/` the tool
- `bin/doc-links`
```

- One bullet per path; the first code span is the path. A directory ends with `/` and covers every file under it.
- Paths are under `apps/`, `packages/`, `tooling/` or `bin/` and exist.
- Invariants and ADRs state rules, not implementations: code and tests cite them, and they carry no `Covers`.
- A page with `Covers` stays anchored to its code: a commit that changes covered code changes the page too, or says
  why the page still holds.

## Commands

Run through `bin/doc-links`. Output is compact and agent-first; exit codes follow [[best-practices/tooling-cli.rules]]: 0 the
links hold, 2 findings the caller must fix, 1 the tool itself failed. Full command reference, options and sample
output live in [[tooling/doc-links.tool]].

| Command             | Does                                                                                                                                                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `check [paths]`     | Every check below, on the given paths or the whole repository                                                                                                                     |
| `refs <path or id>` | For code: what it cites and which docs cover or mention it. For a doc or id: which code and tests cite it and what it covers                                                      |
| `matrix`            | Every invariant, spec heading, screen and ADR with its count of citations from code and from tests; names the invariants no test cites                                            |
| `kinds`             | Prints every doc kind: its file pattern, folders and what it requires ([[repo/doc-kinds]])                                                                                        |
| `graph`             | Replaces its previous edges (`"_origin": "doc-links"`) in `graphify-out/graph.json` with `cites`, `validates`, `covers` and `references` edges, all `EXTRACTED` at confidence 1.0 |
| `commit <msg-file>` | The commit gate (below)                                                                                                                                                           |

`graph` connects the nodes graphify made. When a link ends at a tracked file graphify made no node for (a workspace
manifest, a `clippy.toml`, a git hook with no extension), `graph` adds a file node for it, marked
`"_origin": "doc-links"` like its edges and replaced on every run, so no checked link is dropped. Its summary row is
`added=<edges> removed=<edges> nodes=<file nodes added> unplaced=<links still without a node>`.

The scan skips what git ignores (it reads the `.gitignore` files; no git process runs) and `.gitkeep` placeholders, so a
directory under `## Covers` links only the files that are part of the repository.

`refs` reads the working tree and needs no graph.

## What `check` verifies

1. Every citation in code and tests resolves: the doc exists, the heading slug exists, the id exists.
2. Every `## Covers` path exists. The source scan includes workflows, actions and scripts under `.github/`.
3. Every file under `docs/` has the frontmatter with `description` and `tags` (a list), and only the keys its kind
   allows: `name` in a `SKILL.md`, `status` in an ADR, `superseded_by` and `commit` in a `.old/` entry (INV-DOCS-002).
4. Every wikilink target page exists, and every `#heading` in a wikilink exists on that page. Wikilinks inside code
   spans and fenced blocks are examples and are skipped.
5. Every doc is listed in its scope `SKILL.md`, and every scope `SKILL.md` in `docs/SKILL.md` (INV-DOCS-003).
6. Every `.old/` entry has `superseded_by` pointing at an existing page or heading and `commit` holding a short hash;
   `pending` is accepted only in a file that is uncommitted or added by `HEAD` ([[repo/history]]). This is the only
   check that runs git, and only when an entry says `pending`.
7. Every doc has the structure of its kind, named by its file ending: required headings in order, item headings and
   lines, file name, folder ([[repo/doc-kinds]]). Findings `E_PLACE`, `E_NAME`, `E_META`, `E_KIND`, `E_ITEM`,
   `E_PENDING` name the kind and its heading on that page.
8. Every workspace under `tooling/<name>/` (holding `Cargo.toml` or `package.json`) and every executable in `bin/` has
   `docs/tooling/<name>.tool.md` listing its path under `Covers` (`E_TOOL_DOC`, `E_TOOL_COVERS`), every `tooling/`
   workspace has a non-empty `README.md` (`E_TOOL_README`), and every `.tool.md` names an existing tool
   (`E_TOOL_ORPHAN`; INV-TOOLING-006, INV-TOOLING-007).

## The commit gate

`.husky/commit-msg` runs `bin/doc-links commit <msg-file>` after commitlint. It exits 2 and blocks the commit when:

1. a staged file has a citation that does not resolve, or a citation anywhere points at a staged doc and no longer
   resolves (a heading renamed, a doc moved or deleted);
2. a staged doc fails checks 3 to 7 above; or
3. a staged path is covered by a page that is not staged.

Check 3 is waived by a trailer in the commit message, for a change that leaves the contract as it is (formatting, a
rename inside a module, a fix that makes code match the doc):

```text
refactor(doc-links): split the slug parser out of the scanner

Spec-unchanged: docs/repo/doc-links.md describes the same forms; only the file layout changed.
```

The gate reads staged files at their staged content and every other file as it is on disk. Docs and code cite each
other, so a scoped commit may stage one side while the other is still unstaged; a citation blocks only when its target
exists nowhere. `bin/doc-links check` stays strict on the whole working tree.

Merges are not checked. Fix the link or update the doc; never bypass the gate with `--no-verify`.

## Speed and where it runs

| Where                       | Runs                                                               |
| --------------------------- | ------------------------------------------------------------------ |
| `.husky/commit-msg`         | `bin/doc-links commit <msg-file>`                                  |
| Lint task                   | `bin/doc-links check`                                              |
| `.husky/graphify-update.sh` | `bin/doc-links graph` in the background, after `graphify update .` |

The commit gate scans only staged files and their citers; `check` caches per-file results by content hash, so both
stay well under a second (INV-TOOLING-002).

## Finding links both ways

```sh
bin/doc-links refs tooling/doc-links/src/main.rs   # what it cites; which docs cover or mention it
bin/doc-links refs docs/repo/history.md            # which code and tests cite it; what it covers
bin/doc-links refs INV-DOCS-004                     # which code and tests cite an invariant
bin/doc-links matrix                                # every rule with its code and test citations
```

`graphify explain "<symbol>"` lists the docs a function cites once `graph` has run ([[repo/graphify]]).

## Covers

- `tooling/doc-links/` the tool
- `bin/doc-links`
