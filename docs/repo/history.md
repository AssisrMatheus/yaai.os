---
description: The .old pattern; where superseded docs go, how they are named, and how they point at their replacement and commit.
tags: [repo, docs, history]
---

# Documentation history: `.old/`

Each scope folder under `docs/` may have a `.old/` folder. It holds older text of rules, scopes, ADRs, invariants,
specs, domain rules and plain docs that was overwritten for any reason. `.old/` is the history of the documentation
(INV-DOCS-004, ADR 0005).

## When text moves to `.old/`

When the user driving the agent decides a change that overwrites or rewrites text under `docs/`, the agent moves the
old text into the `.old/` folder of the scope that held it, in the same commit as the new text.

- Rewording that keeps the meaning (a typo, a broken link, formatting) stays in git history only.
- A change of meaning (a rule, a decision, a behaviour, a definition) moves the old text.
- A deleted page with no successor still moves; its `superseded_by` names the scope index.

## Who reads it

`.old/` explains why and how things changed. It is never a source of rules: its weight is very low. Read it only when
the task asks for the history of a rule. The current page never points at `.old/`; the link runs one way, from the
old text to the new.

Obsidian hides dot-folders, and `.old/` is listed in `.graphifyignore`, so it stays out of the vault views and out of
the graphify graph.

## File name

`docs/<scope>/.old/<name>.<YYYY-MM-DD>.old.md`

- `<name>` is the replaced file's name without `.md` (`hooks`, `0004-obsidian-vault-and-skill-indexes.adr`, `SKILL`).
- `<YYYY-MM-DD>` is the day of the replacing commit.
- A second replacement of the same file on the same day adds `-2`, `-3`: `hooks.2026-11-26-2.old.md`.
- A page in a nested folder keeps its scope: `docs/repo/turborepo/.old/SKILL.2026-11-26.old.md`.
- The `.old.md` ending makes it an old entry, whatever kind the replaced page had ([[repo/doc-kinds#old]]).

## Frontmatter

```yaml
---
description: <the old description, unchanged>
tags: [<old tags>, old]
superseded_by: '[[repo/hooks#Exit codes]]'
commit: 1a2b3c4
---
```

- `superseded_by`: a quoted wikilink to the page, or the heading, that replaced the text.
- `commit`: the short hash of the commit that replaced it (see the next section).
- Keep the old body as it was; add nothing to it.

## Filling in the commit

A commit cannot contain its own hash. The move and the replacement land in one commit with `commit: pending`; the next
commit fills the hash.

1. Write `commit: pending` in the new `.old/` file and commit the change.
2. Look up the hash: `git log --diff-filter=A -1 --format=%h -- docs/<scope>/.old/<file>`.
3. Replace `pending` with that hash in the next commit (any commit; a dedicated one is
   `docs(<scope>): record the superseding commit`).

Why this procedure: the `git log --diff-filter=A` lookup returns exactly the commit that added the file, with no amend
or rewrite of history. `bin/doc-links check` accepts `pending` only in a file that is uncommitted or added by `HEAD`,
so a forgotten hash blocks the commit after next ([[repo/doc-links]]).

## When one section is replaced

Move only the replaced section, with its heading, into `<name>.<YYYY-MM-DD>.old.md`. Set the H1 to
`# <Page title>: <Section heading>` and point `superseded_by` at the new heading (`"[[repo/hooks#Exit codes]]"`). When a
change replaces several sections of one page, move each one under its own H2 in the same file and point
`superseded_by` at the page.

## Worked example

On 2026-12-03 the user decides that tool findings exit 2 instead of 1. The agent rewrites the `Exit codes` section of
`docs/repo/hooks.md` and creates `docs/repo/.old/hooks.2026-12-03.old.md`:

```markdown
---
description: How hooks and tools choose their exit codes.
tags: [repo, hooks, old]
superseded_by: '[[repo/hooks#Exit codes]]'
commit: pending
---

# Hooks: Exit codes

<the old section, word for word>
```

The commit `docs(repo): findings exit 2` contains both files. `git log --diff-filter=A -1 --format=%h --
docs/repo/.old/hooks.2026-12-03.old.md` prints `9f8e7d6`; the next commit sets `commit: 9f8e7d6`.

## Checks

`bin/doc-links check` fails when a `.old/` file lacks the `.old.md` ending or the date in its name, lacks
`superseded_by` or `commit`, when `superseded_by` points at a page or heading that does not exist, when `commit` is
neither a short hash nor `pending`, or when `pending` outlived the commit after the one that added the file
([[repo/doc-links]], [[repo/doc-kinds#old]]).
