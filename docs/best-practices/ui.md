---
description: Framework-neutral rules for where a component comes from, how it is composed and named, and how it is made ready for people and agents to operate.
tags: [best-practices, ui]
---

# User interface components

No UI framework is chosen; these rules hold for any. State and effects are in [[best-practices/ui-state]], links and
transitions in [[best-practices/navigation]], styling in [[best-practices/design-tokens]], screens are specified in
`docs/screens/` first (ADR 0006).

## Where a component comes from

### BP-UI-001: Look before writing a component

Walk this order and stop at the first yes: the app already has it; the chosen primitive layer ships it; a shared behaviour package has a primitive for it; write a primitive from shared behaviours; plain markup.
Why: a second button or select in one app is the most expensive UI mistake.
Check: review

### BP-UI-002: Build widgets from the primitive layer

Build dialogs, menus, listboxes, tabs, comboboxes and carousels from the chosen primitive layer, never from bare elements with roles.
Why: focus trapping, dismissal and ARIA wiring are what hand-rolled versions get wrong.
Check: review

### BP-UI-003: Use one behaviour layer

Composite widgets come from one primitive layer; a second headless library is not added beside it.
Why: two layers mean two focus and dismissal models on one screen.
Check: lint

### BP-UI-004: Wrap primitives once in app components

The app's component is a thin, named, styled layer over the primitive's parts; screens import the app component, never the primitive directly.
Why: styling and naming exist once.
Check: lint

### BP-UI-005: Import behaviours, never copy them

Use a shared behaviour by importing it; never copy its code into the app.
Why: a copy misses every later fix.
Check: review

### BP-UI-006: Split presentation from data

A presentational component renders from its inputs and emits typed actions; the container that reads app state and services lives in the app and passes props down. A presentational component never imports app state, services or another app's code.
Why: the component renders the same in a story, a test and the app.
Check: lint

## Composition and props

### BP-UI-007: Compose parts that share state

Use compound components (named parts sharing implicit state) when parts vary in order or presence; keep a simple fixed component as one piece.
Why: composition scales without a prop for every part.
Check: review

### BP-UI-008: Prefer children and slots to configuration objects

Pass content as children or named slots, not as nested configuration props.
Why: callers compose markup freely and the component stays small.
Check: review

### BP-UI-009: Use explicit variants, not boolean props

Express visual choices as named variant axes (`variant="primary"`, `size="lg"`), never as stacks of booleans.
Why: impossible combinations disappear and the axis is declared once.
Check: review

### BP-UI-010: Name props consistently

Use the same prop names across components (`disabled`, `open`, `value`, `onOpenChange`), positive booleans, and `on*` for handlers.
Why: a caller who learned one component knows them all.
Check: review

### BP-UI-011: Extend the native element's props

A component accepts its root element's native attributes, spreads the rest last, merges class names, and forwards events, focus and references to the real control.
Why: callers add `aria-*`, `data-*` and handlers without the component growing a prop for each.
Check: review

### BP-UI-012: Choose safe defaults

Defaults suit the common case and fail safe (a button's `type` is `button`, not `submit`).
Why: a default that submits a form by accident is a bug in every caller.
Check: lint

### BP-UI-013: Render as another element through one convention

When a component must render as a different element (a link styled as a button), use one project-wide mechanism for it.
Why: one rule covers primitives and app components alike.
Check: review

### BP-UI-014: Mark parts with data attributes

Every part carries a `data-slot`, and components with variants expose `data-variant` and `data-size`.
Why: parents and styles target parts without depending on class names.
Check: review

### BP-UI-015: Extract repeated markup at the third copy

Extract a component when the same markup appears in three places; not before.
Why: premature components freeze the wrong interface.
Check: review

### BP-UI-016: Bound complex regions with error boundaries

Wrap each independently failing region in an error boundary with a local recovery.
Why: one failing panel must not blank the screen.
Check: review

## Layout

### BP-UI-017: Lay out responsively in CSS

Switch layouts with media and container queries, never by detecting the viewport in code.
Why: code-driven layout renders wrong first and shifts on hydration.
Check: review

### BP-UI-018: Let content set the height

Size text containers by padding and content, never by fixed heights.
Why: translations and long titles get clipped.
Check: review

### BP-UI-019: Use current platform features with a stated fallback

Target current browsers and engines; where one lacks a feature, the component degrades to a fallback stated in its doc, without polyfills.
Why: modern features remove code, and the fallback stays deliberate.
Check: review

### BP-UI-020: Keep refresh free of flashes

Set initial state before the first render (from storage or the server) so a reload shows no flash of wrong content or theme.
Why: a flash on every reload reads as broken.
Check: review

### BP-UI-021: Keep hover-revealed content in the document

Content revealed on hover (submenus) exists in the document and is reachable by keyboard; hover only shows it.
Why: assistive technology and keyboards cannot hover.
Check: review

## Ready for people and agents

### BP-UI-022: Name every control

Every control has an accessible name from the translations ([[best-practices/accessibility]]).
Why: screen readers and agents find a control by its name.
Check: lint

### BP-UI-023: Make controls operable by agents

Controls an agent should reach are primitive-layer parts with names; actions that need a person's confirmation are marked; a primitive the app writes declares its actions.
Why: agents operate the same controls people do.
Check: review

### BP-UI-024: Check new UI through its accessibility tree first

After building or changing a component, read the screen's accessibility or agent snapshot to check every control is listed, named and operable; use screenshots for what the tree cannot show (layout, colour, focus rings, motion).
Why: the snapshot shows unreachable controls at a fraction of a screenshot's cost.
Check: review

### BP-UI-025: Try worst-case content before calling it done

Exercise missing values, long unbroken labels, dense lists, narrow sizes, large text, right-to-left layout and failed requests.
Why: a happy-path screenshot hides clipping and overflow.
Check: review

### BP-UI-026: Test touch surfaces on a real device

A surface meant for touch gets a pass on a real device.
Why: emulators miss sticky hover, tap delay and virtual keyboards.
Check: review

### BP-UI-027: Title every screen

Each screen sets a unique, descriptive title through one title template; the app name appears once.
Why: titles identify screens in history, tabs and assistive technology.
Check: review

## Covers

- `apps/`, `packages/` (components, screens and shared presentation code)

## References

- [Async UI and agent diagnostics](https://www.solidjs.com/blog/solid-2-0-the-long-hold/)
