---
description: Fill-in template for a screen or subview doc; copy it to docs/screens/<name>.screen.md.
tags: [screens, template]
---

# Screen template

Copy this file to `docs/screens/<name>.screen.md`, replace every `<...>`, delete the guidance lines in italics, and
add a row to [[screens/SKILL]]. Write behaviour only: the doc must be implementable as written in Rust, a web page, a Swift app
or a TUI. Light pseudocode may explain a rule; it never picks a framework, a type or a file layout. Use the terms of
[[glossary]].

The copied file starts with:

```markdown
---
description: <What the user does on this screen, in one sentence.>
tags: [screens, <area>]
---

# <Name>
```

## Name

_`<Name>`, and its kind: screen, or subview of `<parent screen>` (drawer, dialog, drawer sheet, stacked view)._

## Description

_Two or three lines: what the user comes here for and what they leave with._

## View

_A relative reference to a screenshot (`![<name>](./assets/<name>.png)`). When none exists yet, draw the main elements
with their names and their position relative to each other and to the window. The drawing below only shows the
form; replace it:_

```text
+--------------------------------------------------------------+
| [Title]                                        [Element A]   |
|                                                              |
|  [List: <name>]                                              |
|  [Item] [Item] [Item] [Item] ->   (scrolls right)            |
|                                                              |
|  [Element B: <name>]                                         |
|                                                              |
| [Hint bar: <input> <action> ...]        (bottom, full width) |
+--------------------------------------------------------------+
```

## Data

_What it shows, where each piece comes from in domain terms (the household's library, an app, a setting), and how
fresh it must be._

## States

| State   | When          | What the user sees                       |
| ------- | ------------- | ---------------------------------------- |
| Loading | `<condition>` | `<what is shown>`                        |
| Empty   | `<condition>` | `<what is shown and the action offered>` |
| Error   | `<condition>` | `<message and the recovery action>`      |
| Ready   | `<condition>` | `<the view above>`                       |

## Lifecycle

_What happens when the view is created, shown, hidden (another view on top), resumed and closed: what it loads, what
it keeps, what it releases._

## Props

| Direction | Name     | Meaning                                                  |
| --------- | -------- | -------------------------------------------------------- |
| Receives  | `<name>` | `<what the parent or the navigation passes in>`          |
| Returns   | `<name>` | `<what the view hands back when it closes, if anything>` |

## Events

| Direction  | Event     | Payload  | When                                           |
| ---------- | --------- | -------- | ---------------------------------------------- |
| Listens to | `<event>` | `<data>` | `<what the view does in response>`             |
| Fires      | `<event>` | `<data>` | `<which interaction or state change fires it>` |

## Hooks and callbacks

_The points where the surrounding app runs logic: before the view opens, after it closes, on selection, on confirm.
Name each and say when it runs and what it may change._

## Behaviours

_One entry per interaction. A short name, then two lines on what it does._

**<Behaviour name>**
<What the user does, with which input.>
<What changes on screen, and what event or navigation follows.>

```text
on Back:
  if a subview is open: close it
  else: return to the previous screen
```

## Navigation

- Reached from: `<screens or subviews that open this one, and how>`
- Leads to: `<screens or subviews this one opens, and with which behaviour>`
- Parent: `<the screen this subview belongs to, or none>`
- Child subviews: `<subviews this view opens>`

_Link each named view by its wikilink once its doc exists._

## Input and accessibility

| Input             | Mapping                        |
| ----------------- | ------------------------------ |
| Remote            | `<directions, OK, Back, Menu>` |
| Keyboard          | `<keys>`                       |
| Gamepad           | `<buttons>`                    |
| Touch and pointer | `<gestures>`                   |

_Where focus starts, how it moves and wraps, what Back does at the edge, and what is read aloud for each focused
element. Every action reachable by touch is reachable by remote, and the reverse._

## Covers

_Only once code exists: keep this last section and list the repository paths that reproduce this view, one per
bullet, in backticks. Each of those files cites this doc with `screen: docs/screens/<name>.screen.md`. Until there is
code, delete this section: an empty one fails the check. Every other section above is required, in this order
([[repo/doc-kinds#screen]])._
