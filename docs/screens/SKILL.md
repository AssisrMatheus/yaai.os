---
name: yaai-screens
description: One implementation-neutral doc per screen and subview, written before any code. Use when adding, changing or implementing a screen, a subview, or an interaction on one.
tags: [screens, index]
---

# Screens

A **screen** is one full view of an app, the equivalent of a page on a website or a stack on a mobile app. A
**subview** (drawer, dialog, drawer sheet, stacked view) is also a view and has its own doc here. The doc comes first;
code is a reproduction of it and cites it with `screen: docs/screens/<name>.screen.md` (INV-DOCS-008, ADR 0006).

- One file per screen or subview: `docs/screens/<name>.screen.md`, kebab-case, copied from [[screens/_template.screen]].
  The `.screen.md` ending makes it a screen doc: `bin/doc-links check` requires every section of the template as an
  H2, in order ([[repo/doc-kinds#screen]]).
- A screen doc decides behaviour, never implementation: the same doc must be implementable in Rust, a web page, a
  Swift app or a TUI. Light pseudocode may explain a behaviour.
- When behaviour changes, the doc changes first; the old text moves to `docs/screens/.old/` ([[repo/history]]).

No screen exists yet.

## Which file to read

| You want to | Read |
| --- | --- |
| Write a new screen or subview doc | [[screens/_template.screen]] |
| Build the UI from a screen doc | [[best-practices/ui]] |
