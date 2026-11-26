---
description: Decide to structure docs/ as an Obsidian vault linked with wikilinks and indexed by SKILL.md files.
tags: [adr, docs]
status: accepted
---

# The encyclopedia is an Obsidian vault indexed by SKILL.md files

On 2026-11-26 the owner decided to organize `docs/` as an Obsidian vault using vault-root wikilinks (`[[scope/page]]`) and a `SKILL.md` routing index in every scope folder (`INV-DOCS-003`, `INV-DOCS-005`, [[repo/documentation]]). Wikilinks work natively in Obsidian and produce clean graph edges in [[repo/graphify]], while `SKILL.md` indexes carry structured frontmatter triggers so agents route by task without loading the whole tree.

## Considered options

- **Plain Markdown relative links and `index.md` files**: rejected because relative paths break when files move, lack native Obsidian vault ergonomics, and separate human browsing from agent trigger routing.

## Consequences

- `docs/.obsidian/app.json` is committed with `newLinkFormat: absolute` and `useMarkdownLinks: false`; workspace UI files stay git-ignored.
