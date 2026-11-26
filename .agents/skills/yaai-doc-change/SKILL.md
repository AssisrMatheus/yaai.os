---
name: yaai-doc-change
description: Procedure for creating or changing anything under docs/ (a page, scope, invariant, ADR, spec, screen doc or the glossary). Use when adding a doc, editing a rule or decision, superseding text, or adding a scope.
---

# Change the docs

Facts behind each step: `docs/repo/documentation.md` (format, links, indexes), `docs/repo/doc-kinds.md` (file
endings and the structure each kind requires), `docs/repo/history.md` (`.old/`), `docs/repo/doc-links.md` (checks). For a screen or subview, follow the `yaai-screen` skill.

1. **Find the page.** Open `docs/SKILL.md`, follow the row for the task to one scope `SKILL.md`, then to one page.
   For "where is X", query the graph (`graphify query "<X>"`). Done when you hold the page that answers the question,
   or know none exists and which scope owns it.
2. **Confirm the decision.** A doc changes only because the user decided it. Done when the change you are about to
   write matches what the user asked, or you have asked.
3. **Move superseded text.** If the change alters a meaning, copy the old page or section into
   `docs/<scope>/.old/<name>.<YYYY-MM-DD>.old.md` with `superseded_by: "[[<scope>/<page>#<heading>]]"` and
   `commit: pending`. Done when every replaced meaning has an `.old/` entry; rewording that keeps the meaning needs none.
4. **Write the page.** Name it with its kind's ending (`<name>.screen.md`, `NNNN-<slug>.adr.md`,
   `<domain>.invariants.md`, `<name>.spec.md`, `<topic>.rules.md`; `bin/doc-links kinds` lists them) and give it the
   kind's required sections. Frontmatter `description` and `tags` (a list; `name` first in a `SKILL.md`), H1, content in
   glossary terms, wikilinks as `[[<scope>/<page>]]`, then `## Covers` if it governs code and `## References` with
   the owner's references (markdown URLs). Done when the page holds one subject and stays under about 200 lines.
5. **Update the index.** Add or fix the page's row in its scope `SKILL.md`; a new scope gets a row in
   `docs/SKILL.md`. Done when every new or renamed page has exactly one row.
6. **Update citers.** Run `bin/doc-links refs docs/<path>.md`; fix every citation and wikilink to a renamed heading or
   moved page. Done when `refs` lists no unresolved citer.
7. **Check.** Run `bin/doc-links check docs/`. Done when it exits 0.
8. **Fill the commit hash** after committing a new `.old/` entry: run
   `git log --diff-filter=A -1 --format=%h -- <old file>` and replace `pending` in the next commit.
