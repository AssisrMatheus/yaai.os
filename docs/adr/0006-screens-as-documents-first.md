---
description: Decide that every screen and subview is specified as an implementation-neutral document before UI code is written.
tags: [adr, docs, screens]
status: accepted
---

# Every screen is a document before it is code

On 2026-11-26 the owner decided that every user-facing screen and subview must have an implementation-neutral document under `docs/screens/` before its UI code is written (`INV-DOCS-008`, [[screens/SKILL]]). Separating screen states, flows, and content from any UI framework keeps product intent durable across stack or component rewrites and lets agents verify UI code against a clear contract.

## Considered options

- **Design screens directly in UI component code**: rejected because product requirements become entangled with framework details and cannot be reviewed independently before implementation.

## Consequences

- UI code implementing a screen cites `screen: docs/screens/<name>.md` and stays checked by `bin/doc-links`.
