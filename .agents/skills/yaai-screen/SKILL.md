---
name: yaai-screen
description: Procedure for documenting a screen or subview before code exists, and keeping its doc and code in step. Use when adding a screen or subview (drawer, dialog, sheet, stacked view), changing how one behaves, or implementing one in code.
---

# Document and build a screen

Facts: `docs/screens/SKILL.md` (what a screen is, the rules), `docs/screens/_template.md` (the sections),
`docs/repo/doc-links.md` (citations). The doc always comes first; code reproduces it. Use the `yaai-doc-change`
skill for the general doc steps (index, `.old/`, check).

## New screen or subview

1. **Name it** with the glossary's terms (`docs/glossary.md`); add a term there if the view introduces one. Done when
   the kebab-case file name `docs/screens/<name>.md` is free.
2. **Copy the template** to `docs/screens/<name>.md` and fill every section. Write behaviour only, implementable in
   Rust, a web page, a Swift app or a TUI. Each subview it opens gets its own doc. Done when no `<...>` placeholder
   or italic guidance line remains, and every interaction has a Behaviours entry and an input mapping.
3. **Index it.** Add a row to `docs/screens/SKILL.md`; link parent and child views both ways in Navigation. Done when
   `bin/doc-links check docs/screens/` exits 0.
4. **Stop for review.** Show the doc to the user before writing code. Done when the user approves it.

## Behaviour change

1. **Change the doc first.** Move the replaced sections to `docs/screens/.old/` and edit the screen doc. Done when the
   doc describes the new behaviour and `bin/doc-links check` exits 0.
2. **Then change the code** to match it.

## Implement or edit screen code

1. **Cite the doc** in the comment above the view's entry item: `screen: docs/screens/<name>.md`; cite a single rule
   with `spec: docs/screens/<name>.md#<heading-slug>`. Done when every file that renders the view cites it.
2. **List the code** in the doc's `## Covers`. Done when `bin/doc-links refs docs/screens/<name>.md` shows the
   citations and the covered paths match.
3. **Test the behaviours.** Each test names the behaviour or cites the doc heading it pins. Done when every Behaviours
   entry has a test, or the user agreed to leave it untested.
