---
description: The domain language of yaai.os; names in code and docs use these terms.
tags: [glossary]
---

# Glossary

Names in code and docs use these terms (INV-DOCS-009). Add a term when a change introduces one; settle a conflicting
or fuzzy term here before using it.

## Product

**Home OS**: The software a household runs on the screen of its media center as its starting point. yaai.os is a home
OS.
_Avoid_: launcher, dashboard

**Media center**: The device, usually a TV or a computer attached to one, where a household watches, listens and plays.

**Screen**: One full view of an app, the equivalent of a page on a website or a stack on a mobile app. Every screen
has its own doc under `docs/screens/`.
_Avoid_: page, route

**Subview**: A view that opens on top of or inside a screen, such as a drawer, a dialog, a drawer sheet or a stacked
view. A subview has its own doc, like a screen.
_Avoid_: modal, popup

## Documentation

**Encyclopedia**: The `docs/` folder as a whole: the source of truth for rules, specs, invariants, decisions and
screens, which code reproduces.

**Scope**: A folder under `docs/` that owns one area and has its own index.

**Index**: The `SKILL.md` of a scope, which routes a task to one page. `docs/SKILL.md` is the root index.
_Avoid_: table of contents, README

**Doc kind**: The type of a doc, named by its file ending (`.screen.md`, `.adr.md`, `.invariants.md`, ...), which
fixes the structure the doc must have. A page with no kind ending is a plain page.
_Avoid_: doc type, template type

**Invariant**: A rule that always holds, identified as `INV-<DOMAIN>-NNN`.

**ADR**: An architecture decision record: one decision that is hard to reverse, surprising without context or a real
trade-off, identified as `ADR NNNN`.
_Avoid_: decision log

**Spec**: A doc page that states, as checkable rules, what a piece of code must do.

**`.old`**: The folder of a scope that keeps superseded doc text, each entry pointing at its replacement and the commit
that replaced it.
_Avoid_: archive, deprecated

**Citation**: A reference from code or a test to the doc, invariant or ADR that governs it, written in the comment
above the item.

**Covers**: The section of a doc that lists the code paths it governs; the reverse of a citation.

**Best-practice rule**: One checkable coding rule in `docs/best-practices/`, identified as `BP-<AREA>-NNN`, with a
reason and the way it is checked (lint, test or review).
_Avoid_: guideline, convention

## Repository

**Workspace**: One unit of the monorepo with its own manifest, of one kind: app, package or tool.

**App**: A workspace under `apps/` that runs as a product a user starts. An app never imports another app.

**Package**: A workspace under `packages/` that apps and other packages import.

**Tool**: A workspace under `tooling/` that runs as a CLI for development and is never shipped.
_Avoid_: script, utility

**Entry point**: An executable POSIX `sh` file under `bin/` that invokes a tool or runs a small standalone command.

**Vendor**: External source code copied into `vendor/` to read, never edited and never committed.

**Data store**: Anything that keeps the product's data across runs, such as a database or an embedded store. None
is chosen yet; its shape will be a data-model spec.
_Avoid_: DB, persistence layer

## Agents

**Agent runtime**: A program that runs a coding agent in this repository (Claude Code, Codex, Antigravity, ...). Each
reads `.agents/` through its own folder of links.
_Avoid_: harness, client

**Skill**: A `SKILL.md` with a `name` and a `description` saying when to use it. A runtime skill in `.agents/skills/`
holds a procedure; a docs index is also a skill file but only routes.

**Hook**: A command an agent runtime or git runs on an event (before a tool call, at commit), which can block the
action by exiting 2.

**Self-review**: The adversarial review an agent runs on its own diff against the best-practice rules before
reporting a code change done. Its findings block until each is fixed or justified.

**Code mode**: Running many tool calls as one script instead of one call per turn. psychoscript is the code mode of
this repository.
