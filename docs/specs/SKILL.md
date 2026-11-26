---
name: yaai-specs
description: Specs of code that has no better scope. Use when writing or changing the rules for a piece of code that no other scope owns, or when implementing code a spec here covers.
tags: [specs, index]
---

# Specs

A spec states what a piece of code must do. It lives here only when no other scope owns the code: screens go to
[[screens/SKILL]], repository tooling to [[repo/SKILL]], always-true rules to [[invariants/SKILL]].

A spec page contains, after the frontmatter and H1:

1. Two or three lines on what the code is for.
2. Its rules as checkable statements, one per bullet or heading ("`<command>` exits 2 when ..."), each one a test can
   pin. Code cites a rule by its heading: `spec: docs/specs/<name>.md#<heading-slug>`.
3. `## Covers`: the code paths it governs ([[repo/doc-links#Listing covered code]]).
4. `## References` when the owner gave references ([[repo/documentation#References footer]]).

No spec exists yet.

## Which file to read

| You want to | Read |
| --- | --- |
| Write or change a spec page | [[repo/documentation#Add a page]] |
| Cite a spec from code or a test | [[repo/doc-links#Citing docs from code]] |
