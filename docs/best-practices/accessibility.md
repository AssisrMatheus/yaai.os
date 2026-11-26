---
description: The accessibility rules every screen and control meets, for keyboard, pointer, touch and remote users and assistive technology.
tags: [best-practices, ui, accessibility]
---

# Accessibility

The target is WCAG 2.2 AA. A review flags what the change touches; a full audit of every screen is separate work.

## Semantics and names

### BP-A11Y-001: Target WCAG 2.2 AA

Engineer every screen to WCAG 2.2 AA, and honour reduced motion even though it is AAA.
Why: one target covers legal baselines and the people who use a media centre from a distance.
Check: review

### BP-A11Y-002: Use native elements first

Use buttons for actions and links for destinations; never put click handlers on non-interactive elements (`jsx-a11y/no-static-element-interactions`, `click-events-have-key-events`).
Why: native elements bring focus, keys and the right role for free.
Check: lint

### BP-A11Y-003: Check hand-written widgets against their pattern

A composite widget written in the app declares each part's role and is tested against its ARIA Authoring Practices pattern.
Why: keyboard and ARIA wiring is where custom widgets fail.
Check: test

### BP-A11Y-004: Name every control from the visible label

Every control has a programmatic name from the translations, and the name contains the visible label text (`jsx-a11y/control-has-associated-label`, `label-has-associated-control`).
Why: voice users say what they see.
Check: lint

### BP-A11Y-005: Give an icon-only control exactly one name

An icon-only control is named either by the icon's label or by the control's own label with the icon hidden, never both and never neither.
Why: zero names make it silent; two make it read twice.
Check: review

### BP-A11Y-006: Give images the right alternative

Render source-provided alt text as is, empty alt for decoration, and translated alt for images the code supplies; never a generic or borrowed fallback (`jsx-a11y/alt-text`).
Why: a wrong alternative is worse than an empty one.
Check: lint

### BP-A11Y-007: Write link text that stands alone

Link names make sense out of context; never "click here" or a bare URL.
Why: screen-reader users list links on their own.
Check: review

### BP-A11Y-008: Keep one main heading and ordered levels

Each screen has one level-one heading and no skipped levels; composed sections take their heading level from data or position.
Why: headings are how assistive technology navigates a screen.
Check: review

### BP-A11Y-009: Provide landmarks and skip links

Each screen has a main landmark, and skip links are the first focusable element, pointing at targets that exist and can take focus.
Why: keyboard users jump past repeated navigation.
Check: test

### BP-A11Y-010: Set the language

The root element's language comes from the active locale, and text in another language carries its own language attribute.
Why: speech uses the right pronunciation.
Check: review

### BP-A11Y-011: Title every frame

Every embedded frame has a translated title (`jsx-a11y/iframe-has-title`).
Why: an untitled frame is announced as "frame".
Check: lint

### BP-A11Y-012: Keep ids unique

No two elements share an id that labels, descriptions or relations point to.
Why: duplicate ids break the label and description references.
Check: review

## Focus and keyboard

### BP-A11Y-013: Show focus clearly

Show a visible focus indicator for keyboard focus (focus-visible); never remove the outline without a replacement, and keep it neutral (grey, black or white) at 3:1 against control and background.
Why: an invisible focus strands keyboard and remote users.
Check: review

### BP-A11Y-014: Keep focus clear of sticky elements

Sticky headers and banners never cover the focused element (scroll padding equal to their height).
Why: a hidden focused element is a lost position.
Check: review

### BP-A11Y-015: Keep hidden content out of the tab order

Closed drawers, off-canvas menus and hidden panels are inert or unmounted; `aria-hidden` is never set on focusable content (`jsx-a11y/no-aria-hidden-on-focusable`).
Why: focus disappearing into hidden content is invisible with a mouse and constant with a keyboard.
Check: lint

### BP-A11Y-016: Follow document order for focus

Use the document order for focus; no `tabindex` above 0 (`jsx-a11y/tabindex-no-positive`).
Why: positive values create an order nobody maintains.
Check: lint

### BP-A11Y-017: Manage focus in dialogs

A dialog has a name, moves focus inside on open, traps it, closes on Escape and returns focus to its trigger.
Why: focus otherwise escapes behind the dialog or is lost on close.
Check: test

### BP-A11Y-018: Reach every action by keyboard

Every action is reachable in a logical keyboard order, and moving focus scrolls the focused element into view.
Why: keyboard and remote control users have no pointer.
Check: test

### BP-A11Y-019: Support every input the product supports

Every action works with each supported input: pointer, touch, keyboard and remote; hover enhances but never enables a feature.
Why: a hover-only feature does not exist on touch or remote.
Check: review

### BP-A11Y-020: Offer keyboard equivalents for gestures

Drag and swipe interactions have a keyboard alternative and can be cancelled; a pointer leaving the control never commits an action.
Why: gestures exclude people who cannot perform them.
Check: review

### BP-A11Y-021: Show platform-correct shortcuts

Show keyboard shortcuts beside the actions that implement them, with the platform's modifier names.
Why: shortcuts are discoverable only when shown.
Check: review

## Perception

### BP-A11Y-022: Meet target sizes

Primary touch targets are 44 by 44 CSS px; every target is at least 24 by 24 or has equivalent spacing; grow the hit area with padding, not by scaling the icon.
Why: small targets are missed, especially by older users.
Check: review

### BP-A11Y-023: Meet contrast ratios

Text has 4.5:1 contrast (3:1 at 24 px or 18.66 px bold); component boundaries, focus indicators and meaningful graphics have 3:1.
Why: low contrast is unreadable for many and in bright rooms.
Check: review

### BP-A11Y-024: Pair colour with text or shape

Errors and states are shown by text, icon or shape as well as colour.
Why: colour alone is invisible to colour-blind users.
Check: review

### BP-A11Y-025: Honour zoom and text scaling

Layouts survive browser zoom and large text settings; no text is rendered into images.
Why: low-vision users scale everything.
Check: review

### BP-A11Y-026: Caption speech and pause moving content

Media with speech has captions; content that moves for more than five seconds (carousels, tickers, background video) has a pause control.
Why: deaf users need captions, and motion distracts and disorients.
Check: review

### BP-A11Y-027: Pause timed actions while hidden

A time-limited action freezes while its screen is hidden.
Why: users who switch away must not lose their turn.
Check: review

### BP-A11Y-028: Keep feedback visible and announced

Feedback appears on screen, not only behind hover or in a dialog, and asynchronous status and errors are announced through a live region.
Why: unseen feedback is no feedback.
Check: review

### BP-A11Y-029: Make tooltips accessible

Tooltips open on focus as well as hover, can be dismissed, open after a delay that is skipped for the next tooltip in a group, and nested menus allow diagonal movement without closing.
Why: tooltips must work for keyboard users without getting in a pointer user's way.
Check: review

### BP-A11Y-030: Configure touch handling deliberately

Custom gesture surfaces disable the platform's touch actions they replace, controls disable double-tap zoom, and scrolling and pinch zoom stay available on axes the gesture does not use.
Why: native gestures fight custom ones, and blocking zoom excludes low-vision users.
Check: review

### BP-A11Y-031: Raise low-vision problems as high priority

Contrast, focus visibility, control names, target size, heading structure and error text that does not say how to fix are raised as high priority in any review that touches them, even when the change did not cause them.
Why: these hurt older and low-vision users most, who are a large part of a home media centre's audience.
Check: review

## Covers

- `apps/`, `packages/` (screens, components and their tests)
