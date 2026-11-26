---
name: yaai-docs
description: The yaai.os encyclopedia, source of truth for rules, decisions, specs, screens and how the monorepo works. Use when starting any task, looking for the rule that governs a change, naming something, or documenting a decision, a screen or a tool.
tags: [docs, index]
---

# yaai.os encyclopedia

`docs/` is the source of truth: code reproduces what these pages say (INV-DOCS-001). Each folder is a scope with its
own `SKILL.md` index. Open the one scope your task touches, then the one page its table names. Never read a folder
whole.

## Which file to read

| You want to | Read |
| --- | --- |
| Know what yaai.os is and what it refuses to be | [[product/SKILL]] |
| Check a rule that must never break, before changing anything | [[invariants/SKILL]] |
| Name a thing, or check what a term means | [[glossary]] |
| Follow the working loop for a task, or delegate part of it | [[repo/SKILL]] |
| Add or change a doc, a scope, or supersede a rule | [[repo/SKILL]] |
| Add a workspace, a hook, a skill, a commit, or use rtk, graphify, Turborepo | [[repo/SKILL]] |
| Run a repeatable command, check existing tools, or add or change a tool | [[tooling/SKILL]] |
| Write or review Rust, TypeScript, a CLI, logging, tests or UI code | [[best-practices/SKILL]] |
| Document or implement a screen or subview | [[screens/SKILL]] |
| Find or write the spec of a piece of code | [[specs/SKILL]] |
| Learn why something was decided, or record a decision | [[adr/SKILL]] |
