---
description: How links, prefetching, click feedback and screen transitions behave so navigation is never frozen and never changes response status.
tags: [best-practices, ui, navigation]
---

# Navigation

Links between screens, what happens while the next screen loads, and transitions. Holds for any UI framework.

### BP-NAV-001: Use native navigation semantics

Destinations are links and actions are buttons; navigation keeps history, back and forward, open-in-new and keyboard activation.
Why: people and assistive technology rely on the platform's behaviour.
Check: review

### BP-NAV-002: Route internal links through one link component

Every internal link renders through one app link component that owns locale, active state (`aria-current`), prefetch and click reporting; lower-level link and navigation helpers are imported only there.
Why: one place enforces every link rule, and a lint can check it.
Check: lint

### BP-NAV-003: Prefetch on intent, not on visibility

Prefetch a destination on pointer enter, focus or touch start, not when the link scrolls into view; an explicit per-link choice wins.
Why: visibility prefetch of a grid or menu bursts the source with requests nobody needed.
Check: review

### BP-NAV-004: Give feedback for every click

After a click, show progress and the destination's loading view if the screen has not arrived after a short hold; a click that commits quickly shows nothing.
Why: a frozen screen reads as a missed click.
Check: review

### BP-NAV-005: Report clicks to a store that outlives the link

A link reports its click synchronously to a store that outlives it; the latest click wins and the entry clears on commit, on idle or after a safety timeout.
Why: the link that started a navigation is often unmounted (a closing menu) before the screen arrives.
Check: review

### BP-NAV-006: Show the destination's own loading view

The loading view shown on click is the same component the destination streams while loading, and a test fails when the two disagree.
Why: one skeleton per screen, never two that drift.
Check: test

### BP-NAV-007: Settle on commit

Clear pending navigation feedback when the new screen commits, before it paints.
Why: stale feedback over a loaded screen looks broken.
Check: review

### BP-NAV-008: Decide the response status before any loading feedback

Click feedback is painted on the client; no loading boundary is placed above the check that decides whether the destination exists.
Why: once output streams, the status is sent, so a missing screen would answer success.
Check: review

### BP-NAV-009: Keep the shared shell independent of the route

The shell shared by all screens never reads the current route; per-screen state lives in per-screen scope.
Why: a shell tied to the route re-renders and caches per screen.
Check: review

### BP-NAV-010: Transition screens in per-screen scope

Run screen transitions with the platform's transition mechanism in a scope remounted per screen, never in the persistent shell; query-only changes and refreshes do not animate.
Why: a persistent scope never sees enter and exit.
Check: review

### BP-NAV-011: Respect reduced motion in transitions

Disable screen transitions under reduced motion, and swap without animating where the platform lacks support.
Why: transitions are decoration, never required.
Check: review

### BP-NAV-012: Scroll and focus deliberately on navigation

Scroll to the top when a new screen or its loading view shows, keep anchors clear of sticky headers, and move focus to the new screen's main content.
Why: a navigation that leaves the viewport on blank space or focus on a removed element strands the user.
Check: review

### BP-NAV-013: Apply only the latest navigation's response

A response for an earlier navigation is discarded once a later one started.
Why: out-of-order responses show the wrong screen.
Check: test

### BP-NAV-014: Verify navigation on a production build

Check cold and warm navigation timings and missing-screen statuses on a production build, not a development server.
Why: development servers compile on demand and stream differently.
Check: test

## Covers

- `apps/`, `packages/` (link components, routing and transitions)
