---
description: How styles read design tokens, how themes and variants are declared, where styles live, and the typography and polish rules every surface follows.
tags: [best-practices, ui, design]
---

# Design tokens and styling

No styling stack is chosen; these rules hold for any. Motion values are in [[best-practices/motion.rules]].

## Tokens

### BP-TOKEN-001: Own the tokens in the app

The app defines its tokens in its own stylesheet; no code selects a brand at run time.
Why: the app is the brand, so there is nothing to select.
Check: review

### BP-TOKEN-002: Layer tokens in three tiers

Primitive tokens hold raw values (colour scales, type scale, radii, shadows, durations, easing); semantic tokens name a purpose and point at primitives; component tokens expose one component's knob.
Why: a rebrand, a theme or a designer's tweak changes one tier in one file.
Check: review

### BP-TOKEN-003: Read semantic tokens in components

Components read semantic tokens (`surface-primary`, `fg-muted`), never primitives.
Why: a palette change then touches no component.
Check: review

### BP-TOKEN-004: Write no raw visual values in components

No literal colour, radius, shadow, duration or easing in a component; when a needed value has no token, add the token.
Why: a literal skips every theme.
Check: lint

### BP-TOKEN-005: Keep semantic tokens redefinable

Declare semantic tokens so they resolve through the variable at use time, never inlined into each style.
Why: a theme that redefines an inlined token changes nothing.
Check: review

### BP-TOKEN-006: Drop default palettes

Remove a styling tool's default palette when the app has its own, so off-palette values fail to compile.
Why: off-palette colours never ship by accident.
Check: lint

### BP-TOKEN-007: Create a token for a value with a purpose

Add a token when a theme changes the value, three or more components share it with the same meaning, or a designer would adjust it; a one-off layout value stays inline.
Why: tokens stay meaningful and few.
Check: review

### BP-TOKEN-008: Tokenise appearance, not structure

Colour, type, radius, shadow and motion are tokens; display, position, alignment and cursor are plain styles.
Why: structure does not vary by theme.
Check: review

### BP-TOKEN-009: Add a component token for a parent's knob

When a parent or theme must retune one component, expose a component-level variable on its root defaulting to a semantic token.
Why: the override stays scoped to that component.
Check: review

### BP-TOKEN-010: Name tokens namespace, role, variant

Token names follow namespace, then role, then variant (`color-surface-hover`, `radius-control`).
Why: names sort and search predictably.
Check: review

### BP-TOKEN-011: Fix contrast in tokens

A palette value that fails contrast is fixed with a semantic token whose value passes, never with a per-component override.
Why: the fix then applies everywhere.
Check: review

## Themes and variants

### BP-TOKEN-012: Theme by redefining semantic tokens under one attribute

A theme redefines semantic tokens under one attribute selector (`[data-theme='dark']`) set before the first paint; primitives and components stay as they are.
Why: the renderer resolves the theme, so nothing flashes or mismatches.
Check: review

### BP-TOKEN-013: Ask for a purpose token instead of branching on the theme

Components ask for a purpose token; a class that must differ per theme uses a variant tied to the theme attribute.
Why: theme branches scattered in components drift.
Check: review

### BP-TOKEN-014: Pass a per-user theme down, never through module state

A theme chosen per user or per request is read at the top and passed down.
Why: module state leaks between users.
Check: review

### BP-TOKEN-015: Declare each variant axis once

A class computed from a prop is a declared variant axis with a default, not a ternary in the component.
Why: the axis is visible and exhaustive.
Check: review

### BP-TOKEN-016: Style exposed state from its attribute

State the markup already exposes (`data-open`, `aria-pressed`, `data-invalid`) is styled from that attribute, not from a prop or a toggled class.
Why: one source for the state, and agents changing it get the same styles.
Check: review

### BP-TOKEN-017: Merge classes through one module

Variants and class merging come from one module that knows the custom utilities; class strings are never concatenated by hand.
Why: two mergers resolve conflicts two ways, and hand concatenation leaves conflicts unresolved.
Check: lint

## Where styles live

### BP-TOKEN-018: Colocate component styles

Component styles live with the component; the global stylesheet holds only resets, the base layer, tokens and rich-content overrides.
Why: a component-scoped selector in the global sheet leaks to every screen and outlives the component.
Check: review

### BP-TOKEN-019: Style rich content only inside its container

Rich-text styling applies only to containers of rendered rich content, mapped to the app's type tokens.
Why: on app layout it restyles every descendant.
Check: review

### BP-TOKEN-020: Opt in to native control styling

Base styles for native form controls apply only where a class opts in.
Why: they never leak onto primitive-layer controls.
Check: review

### BP-TOKEN-021: Use the top layer and a fixed stacking scale

Popups use the platform's top layer; elsewhere create stacking contexts with isolation and use a fixed z-index scale, never arbitrary values.
Why: arbitrary z-indexes escalate forever.
Check: review

## Typography and polish

### BP-TOKEN-022: Reference fonts through a token

Fonts load as variables mapped by a token; styles use the token, never a literal family name.
Why: generated family names differ from the literal, and weights silently fall back.
Check: review

### BP-TOKEN-023: Keep weights constant and numbers tabular

Never change font weight on hover or selection; use tabular numbers for values that change (timers, counters, progress).
Why: both cause layout shift.
Check: review

### BP-TOKEN-024: Pair letter spacing with size

Set letter spacing per type size through one text component or token pair; larger text is tighter.
Why: one spacing for every size looks wrong at both ends.
Check: review

### BP-TOKEN-025: Use typographic characters

Write the ellipsis, curly quotes and apostrophes (… ’ “ ”), and balance heading line breaks.
Why: straight substitutes read as unfinished.
Check: review

### BP-TOKEN-026: Draw fine lines deliberately

Use shadows for borders that must blend with varying backgrounds, and a density-aware hairline variable for dividers.
Why: hard borders and 1 px lines look heavy on dense screens.
Check: review

### BP-TOKEN-027: Fade with masks, never over scrolling content

Fade edges with masks or eased gradients; never fade a scrollable list's content, and never replace the page scrollbar (custom scrollbars only in small elements).
Why: fades over scrollable content hide it, and custom page scrollbars break platform behaviour.
Check: review

### BP-TOKEN-028: Respect safe areas

Pad edges with the device's safe-area insets, and set scroll margins equal to sticky headers.
Why: notches, overscan and sticky bars otherwise cover content.
Check: review

### BP-TOKEN-029: Keep decoration out of input

Decorative layers ignore pointer events and are not selectable; illustrations drawn in code have an image role and a label when meaningful.
Why: decoration must not steal clicks or confuse assistive technology.
Check: review

## Covers

- `apps/`, `packages/` (stylesheets, tokens, themes and variant definitions)
