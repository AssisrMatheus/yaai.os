---
description: When to animate, which easing and duration to use, which properties may move, and how motion respects reduced-motion settings and performance.
tags: [best-practices, ui, motion]
---

# Motion

Holds for CSS and script animation alike. Screen transitions are in [[best-practices/navigation.rules]].

### BP-MOTION-001: Decide whether to animate at all

Do not animate actions people repeat all day or start from the keyboard; animate entrances, exits and feedback that give spatial continuity.
Why: motion on frequent actions makes a product feel slow.
Check: review

### BP-MOTION-002: Take easing and duration from tokens

Easing curves and durations are named tokens, never literal values in components.
Why: one place tunes the feel of the whole product.
Check: lint

### BP-MOTION-003: Pick the curve by the kind of motion

Ease-out for anything entering, exiting or responding to an action; ease-in-out for an element moving on screen; plain ease for hover and colour; linear only for constant motion or time displays; never ease-in.
Why: ease-in delays the visible response.
Check: review

### BP-MOTION-004: Keep interface motion short

Interface animations stay under 300 ms; larger surfaces and longer travel take longer, and exits are faster than entrances.
Why: longer motion makes people wait.
Check: review

### BP-MOTION-005: Move paired elements together

Elements that move as a unit (a dialog and its backdrop) share the same easing and duration.
Why: mismatched timing looks broken.
Check: review

### BP-MOTION-006: Animate only transform and opacity

Animate transforms and opacity; never layout properties (width, height, margin, position offsets).
Why: layout animation recalculates every frame.
Check: review

### BP-MOTION-007: Name every transitioned property

List the properties a transition affects; never transition `all`.
Why: `all` animates changes nobody intended and costs performance.
Check: lint

### BP-MOTION-008: Scale from the trigger, never from zero

A popover or menu scales from its trigger's position; a modal stays centred; an element appears from a slightly reduced scale, never from zero.
Why: motion that starts where the action happened reads as cause and effect.
Check: review

### BP-MOTION-009: Acknowledge a press

Buttons respond to a press with a slight scale-down.
Why: immediate feedback confirms the press before the result arrives.
Check: review

### BP-MOTION-010: Give every animation a reduced-motion path

Each animation has its own reduced-motion branch that turns it off, opacity and colour included; a global switch is a backstop, not a substitute.
Why: motion makes some people ill.
Check: review

### BP-MOTION-011: Offer play instead of autoplay under reduced motion

Autoplaying video is muted and inline with a visible pause control, and under reduced motion shows a play control instead.
Why: unexpected motion and sound are hostile.
Check: review

### BP-MOTION-012: Keep gestures interruptible

Drag and swipe follow the pointer directly, can be caught mid-flight, dismiss on distance or velocity, and use springs with subtle bounce for release.
Why: people change their mind mid-gesture.
Check: review

### BP-MOTION-013: Drive per-frame values outside the render cycle

Animate frequently changing values through direct style updates or style variables, never through state updates every frame.
Why: per-frame renders drop frames.
Check: review

### BP-MOTION-014: Interpolate between known states only

Animation moves between known start and end states and never extrapolates or overshoots into a state the data does not hold.
Why: a guessed value shows something untrue.
Check: review

### BP-MOTION-015: Prefer CSS for simple transitions

Use CSS transitions for simple enter, hover and state changes; load a script animation library lazily, with its smallest feature set, and only for interruptible, gesture-driven or sequenced motion.
Why: CSS runs off the main thread and ships no code.
Check: review

### BP-MOTION-016: Animate enter and exit through state attributes

Style a primitive's entering and leaving phases through the attributes it sets for them, rather than wrapping it in a separate presence component.
Why: the primitive stays mounted for the exit and owns its timing.
Check: review

### BP-MOTION-017: Suspend transitions while switching themes

Disable transitions while a theme changes.
Why: otherwise every element animates to its new colours.
Check: review

### BP-MOTION-018: Tie motion to the user's movement

No scroll-triggered fades, scroll hijacking, parallax that does not map one-to-one to scrolling, or auto-advancing carousels.
Why: motion disconnected from input feels out of control.
Check: review

### BP-MOTION-019: Play an intro once per session

An intro animation plays once per session and is skipped afterwards.
Why: repeated intros block repeat users.
Check: review

### BP-MOTION-020: Keep motion effects cheap

Keep blur radii small (under 20 px), and do not animate style variables read by deep trees.
Why: both force expensive repaints on every frame.
Check: review

## Covers

- `apps/`, `packages/` (transitions, animations and gesture handling)
