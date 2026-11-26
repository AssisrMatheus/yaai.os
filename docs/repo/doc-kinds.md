---
description: The doc kinds; which file ending marks each kind, where it lives, and the structure doc-links requires of it.
tags: [repo, docs, tooling]
---

# Doc kinds

A doc's **kind** is given by its file ending (`home.screen.md` is a screen, `0001-docs-first.adr.md` an ADR). Each
kind has a required structure: headings, frontmatter keys, file name, item lines. `bin/doc-links check` verifies it
and says what to add, naming the kind and its heading on this page ([[repo/doc-links]]). The kinds are data in
`tooling/doc-links/kinds.toml`; this page explains them.

## The kinds

| Kind | File | Folder | Requires |
| --- | --- | --- | --- |
| screen | `<name>.screen.md` | `docs/screens/` | Every section of the template as an H2, in order; `Covers` optional, last |
| adr | `NNNN-<slug>.adr.md` | `docs/adr/` | `status`; one H1; an opening paragraph; only `Considered options`, `Consequences` |
| invariants | `<domain>.invariants.md` | `docs/invariants/` | Every H2 is `INV-<DOMAIN>-NNN: <rule>`, with a status line and a body |
| old | `<name>.<YYYY-MM-DD>[-N].old.md` | any `.old/` | `superseded_by` and `commit` |
| spec | `<name>.spec.md` | `docs/specs/` | H2s `Rules` then `Covers`; `References` optional |
| tool | `<name>.tool.md` | `docs/tooling/` | H2s `Synopsis`, `Commands`, `Options`, `Exit codes`, `Output`, `Covers` in order |
| rules | `<topic>.rules.md` | `docs/best-practices/` | Every H3 is `<PREFIX>-NNN: <rule>` with `Why:` and `Check:` lines |
| index | `SKILL.md` | any scope folder | `name`; a `Which file to read` table, a wikilink in every row |
| glossary | `glossary.md` | `docs/` | Every paragraph under an H2 is a `**Term**:` entry; terms unique |

`bin/doc-links kinds` prints the same table from the kinds file, with every pattern (`--json` for one JSON result).

### screen

H2s in this order, none other: `Name`, `Description`, `View`, `Data`, `States`, `Lifecycle`, `Props`, `Events`,
`Hooks and callbacks`, `Behaviours`, `Navigation`, `Input and accessibility`, then `Covers` once code exists. Each
holds text. The file name is kebab-case. Copy [[screens/_template.screen]] ([[screens/SKILL]]).

### adr

Frontmatter `status`: `proposed`, `accepted` or `superseded`. Exactly one H1, then a paragraph (date, who decided,
what, why). The only H2s are `Considered options` and `Consequences`, both optional, in that order. The file name is
`NNNN-<slug>`, the next free 4-digit number. Copy [[adr/0000-template.adr]] ([[adr/SKILL]]).

### invariants

One file per domain: `docs.invariants.md` holds `INV-DOCS-NNN`. Every H2 is `INV-<DOMAIN>-NNN: <rule>`, with
`<DOMAIN>` the file's domain in upper case. Each one has the line `Active · Decided: YYYY-MM-DD, <who>` (or
`Retired · ...`) and a body. An id is unique across every invariants file and never reused: a retired rule keeps its
heading with `Retired` ([[invariants/SKILL]]).

### old

Lives in a `.old/` folder and is named `<name>.<YYYY-MM-DD>.old.md`, `-2`, `-3` for a second replacement the same
day. Frontmatter `superseded_by` (a quoted wikilink to an existing page or heading) and `commit` (a short hash, or
`pending`). `pending` holds only while the file is uncommitted or was added by `HEAD`; this is the one check that runs
git, and only when some entry says `pending` ([[repo/history]]).

### spec

H2 `Rules` (each rule a bullet or an H3 code can cite), then `Covers` with the paths it governs, then `References`
when the owner gave references. Other H2s may sit between them ([[specs/SKILL]]).

### tool

One page per repository tool (`tooling/<name>/` workspace or executable `bin/<name>`). H2s in this order, none other:
`Synopsis`, `Commands`, `Options`, `Exit codes`, `Output`, `Covers`. The file name is kebab-case matching the tool's
directory or `bin/` name, and `Covers` lists the tool's path ([[tooling/SKILL]], INV-TOOLING-006).

### rules

A best-practice page. Every H3 is `<PREFIX>-NNN: <rule>`, one prefix family per page (`BP-RUST`), each id unique
across all rules pages. Each rule holds a `Why: <reason>` line and a `Check: lint`, `Check: test` or `Check: review`
line, plus the rule text. H2s group rules freely; `Covers` and `References` come in that order. A page that links an
external URL ends with `## References` ([[repo/documentation#References footer]]).

### index

A scope `SKILL.md`: frontmatter `name` besides `description` and `tags`, and an H2 `Which file to read` holding a
`| You want to | Read |` table whose every row has a wikilink ([[repo/documentation#Scopes and their index]]).

### glossary

`docs/glossary.md`. After the first H2, every paragraph is an entry `**Term**: what it is.`, optionally followed by
an `_Avoid_: synonyms` line. Each term appears once ([[glossary]]).

## Plain pages

A page whose name matches no kind (`docs/repo/hooks.md`) gets only the generic checks of [[repo/doc-links]]: the
frontmatter `description` and `tags` and no other key, wikilinks, index membership, `Covers` paths.

## Folders

Every page directly in `docs/screens/`, `docs/adr/`, `docs/invariants/`, `docs/specs/`, `docs/tooling/` and
`docs/best-practices/` other than `SKILL.md` is of that folder's kind, and every page in a `.old/` folder is an old
entry. A kind's ending is allowed only in the folders its row names.

## Templates

A file whose name starts with `_` (`_template.screen.md`) is the template of its kind. It carries the kind's
headings, in order, and is exempt from the content checks: file name pattern, section text, item lines, table rows,
entries, `Covers` paths. `matrix` leaves it out.

## Frontmatter keys

| Kind | Keys |
| --- | --- |
| plain page and every kind | `description`, `tags` |
| index | adds `name` (required) |
| adr | adds `status` (required) |
| old | adds `superseded_by` and `commit` (both required) |

## The kinds file

`tooling/doc-links/kinds.toml` holds, per kind: `ending` or exact `file`, allowed `folders`, the `stem` pattern,
allowed and required `keys`, `enums` and `patterns` of values, the `h2` list (a trailing `?` marks an optional one)
with `ordered` and `closed`, page-wide `require` rules, and `items`, `table` or `entries` rules. `[[folder]]` entries
bind a folder to a kind. Folder globs use `*` for one path segment and `**` for any number.

To add a kind or a required section: edit the kinds file, add or change its row and its heading here, then run
`bin/doc-links check`. The Rust engine needs no change. Every kind needs a heading on this page named after it;
`check` reports `E_KINDS` otherwise.

`pending_migration` lists folders that skip every kind check while their pages are renamed to their ending. It is
temporary: remove a folder from it once its pages carry their ending.

## How check applies a kind

1. Skip the page when it sits under a `pending_migration` folder.
2. Find the kind: an exact file name first, then the longest matching ending.
3. Check the folder binding and the allowed folders (`E_PLACE`), then the file name (`E_NAME`).
4. Check frontmatter keys, required keys and values (`E_META`).
5. Check H1, H2 presence, order and strangers, and section text (`E_KIND`).
6. Check items, their lines, their family and unique ids (`E_ITEM`), table rows and entries.
7. Check `commit: pending` against git (`E_PENDING`).

Each finding says what to add or rename and ends with `(kind <name>, docs/repo/doc-kinds.md#<name>)`:

```text
docs/screens/home.screen.md:6: E_KIND add the H2 "Lifecycle" after "States" (kind screen, docs/repo/doc-kinds.md#screen)
```

## Covers

- `tooling/doc-links/kinds.toml` the kinds
- `tooling/doc-links/src/kinds.rs` the engine
