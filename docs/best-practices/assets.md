---
description: How images, artwork, icons, fonts and video embeds are described, sized, loaded, optimised and kept accessible.
tags: [best-practices, ui, assets]
---

# Images, icons, fonts and embeds

A media centre is mostly artwork; these rules keep it sharp, light and stable. Untrusted markup and uploads are also
covered by [[best-practices/security]].

## Images

### BP-ASSET-001: Carry URL, alt text and both dimensions

Image data carries its URL, alt text, width and height, and every read selects all four.
Why: an image without dimensions has no box, and one without alt text has no meaning.
Check: review

### BP-ASSET-002: Fall back to the slot's ratio, never zero

When a source lacks dimensions, use the slot's expected ratio; treat zero as missing.
Why: zero dimensions collapse the box and shift the layout.
Check: review

### BP-ASSET-003: Derive responsive sizes from the layout

Compute the `sizes` value from the layout (container, columns, gutters) through one helper; use the full viewport width only for full-bleed images.
Why: a lazy viewport fraction fetches far more pixels than the slot shows.
Check: review

### BP-ASSET-004: Load images by their position

Preload only the one critical image of the first screen, alone; load the rest of the first screen eagerly, the next region lazily and everything else lazily at low priority. When the critical image differs by screen size, preload neither.
Why: loading follows what the user sees first, never a guess per component.
Check: review

### BP-ASSET-005: Use declared quality tiers

Image quality is one of a few declared tiers (photographs around 75, flat artwork and text around 90), each listed in configuration; a new tier needs a measurement.
Why: every extra tier is another variant to encode and cache.
Check: review

### BP-ASSET-006: Serve the most efficient format first

Offer AVIF before WebP where both are supported.
Why: the first acceptable format wins, so order decides what is served.
Check: review

### BP-ASSET-007: Cap sizes at the source resolution

The ladder of generated widths stops at the largest size the source keeps.
Why: upscaling re-encodes the same pixels at a larger cost.
Check: review

### BP-ASSET-008: Restrict image sources

The image service accepts only configured origins and paths, rejects query strings it does not use, and reaches local addresses only in development.
Why: an open optimiser can be made to fetch anything, under endless URLs.
Check: review

### BP-ASSET-009: Cache long only when URLs change with content

Use long cache lifetimes for images only when a replaced file gets a new URL.
Why: a long-cached URL whose content changes shows the old image.
Check: review

### BP-ASSET-010: Import static images and compress them first

Import static images so tooling reads their dimensions, and compress them before committing.
Why: dimensions come for free and the repository stays small.
Check: review

### BP-ASSET-011: Let resizing services do their own work

Images from a service that resizes by URL are requested at the needed size from that service, not passed through a second optimiser.
Why: two transformations cost twice and lose quality.
Check: review

### BP-ASSET-012: Write a custom loader only without an optimiser

Use a custom image loader only where the built-in optimiser is unavailable, and restrict origins at that service.
Why: a custom loader bypasses the optimiser's origin checks.
Check: review

## Icons

### BP-ASSET-013: Make each icon a component

Each icon is a component wrapping inline vector markup that uses `currentColor`, keeps its `viewBox` and has no fixed width or height; it is sized at the use site.
Why: icons take the surrounding colour and size, tree-shake per import and need no request.
Check: review

### BP-ASSET-014: Decide decorative or labelled at the use site

An icon's props force a choice at each use: hidden as decoration, or given a translated label; the icon's declaration never decides.
Why: the same icon is decoration beside text and the only name of an icon-only button.
Check: lint

### BP-ASSET-015: Optimise vector sources

Strip metadata, editor groups and ids from SVG sources and reduce precision before committing.
Why: every icon ships in the bundle.
Check: review

### BP-ASSET-016: Keep one icon folder

Icons live in one folder per kind (monotone, multicolour), imported by direct path, without a barrel.
Why: icons are found in one place and tree-shake.
Check: lint

### BP-ASSET-017: Import stock icons by name

Import stock icons by name; never import a whole set or look icons up by string; an external icon name maps through an explicit table.
Why: by-name lookup bundles every icon.
Check: lint

### BP-ASSET-018: Render static icons as components

Static icons are components, never `<img>` or the image optimiser; vector markup is never hand-written inside a view.
Why: images cannot take `currentColor`, cost a request and skip the label check.
Check: review

### BP-ASSET-019: Sanitise remote vector markup before inlining

Inline a remote SVG only after fetching it on the trusted side, checking its content type and sanitising it; otherwise render it as an image without inlining.
Why: SVG can carry scripts.
Check: review

## Fonts and embeds

### BP-ASSET-020: Self-host fonts with a matched fallback

Self-host fonts, subset them to the characters and languages used, give them a size-adjusted fallback, and preload only the critical face.
Why: fonts then load fast and swap without shifting text.
Check: review

### BP-ASSET-021: Embed video behind a facade

Embed third-party video through a privacy-preserving host, showing a poster with a play button (with a fallback poster) and mounting the player only on click; other embeds are lazy with a title.
Why: the player's code and tracking load only for people who play.
Check: review

### BP-ASSET-022: Parse pasted media URLs into ids

Parse any accepted form of a pasted media URL into an id in one shared helper that returns nothing for unrecognised input.
Why: one bad paste drops one item instead of breaking a section.
Check: test

## Covers

- `apps/`, `packages/` (image helpers, icon components, fonts and embeds)
