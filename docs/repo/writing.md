---
description: Who each kind of file is written for, and the checkable rules for writing a README or an agent document.
tags: [repo, docs, writing]
---

# Writing for humans and for agents

Two readers, two styles (INV-AGENTS-002):

| File | Reader | Style |
| --- | --- | --- |
| `README.md` (root and every workspace) | A human | Simple and short: what it is, how to start |
| `AGENTS.md`, `.agents/skills/**`, `docs/**` | An agent | Routed by pointers, one subject per page, checkable rules |

A README links to the docs for detail; it never restates them. An agent document never relies on a README.

## README files

Every `apps/*` and `packages/*` workspace gets its own `README.md` when it is created, as does every tool under
`tooling/`. Rules, distilled from the two skills in References:

- [ ] The first line after the title says what the thing is in one sentence.
- [ ] The next block is the first action: one command or path the reader can run now.
- [ ] Multi-step setup is a numbered list; each step is one bounded action.
- [ ] Lists hold at most five items; group or link the rest.
- [ ] Every sentence has a subject doing something (active voice), and names the specific thing.
- [ ] No adverbs, no filler openers ("Let's", "Simply", "This project aims to"), no closing pleasantries.
- [ ] No "not X, it's Y" contrasts, no rhetorical setups, no pull-quote one-liners at paragraph ends.
- [ ] No em dashes. Sentence lengths vary.
- [ ] Errors and caveats state cause and fix, matter-of-fact.
- [ ] Detail lives in `docs/`, linked by repository path.

Size: the root README fits on one screen (about 60 lines); a workspace README stays under 40 lines.

## Agent documents

Rules, distilled from writing-for-agents:

- [ ] **Pointer**: every page and skill is reached by a pointer (an index row, a skill `description`) that states
      what the material is and lists one trigger per distinct case, leading word first.
- [ ] **Two loads**: always-loaded text (`AGENTS.md`, skill descriptions) holds only what every task needs; the rest
      sits behind a pointer.
- [ ] **Hierarchy**: steps first, then reference consulted on demand; reference that only some cases need moves to a
      linked page.
- [ ] **Co-location**: a concept's definition, rules and caveats sit under one heading.
- [ ] **Completion criteria**: every step ends on a checkable condition ("`bin/doc-links check` exits 0", "every
      changed page listed in its index").
- [ ] **Leading words**: use the glossary's terms ([[glossary]]) the same way in prompts, docs and code; reuse a
      known word before coining one.
- [ ] **Positive framing**: state the target behaviour. A prohibition appears only as a hard guardrail, paired with
      the positive target.
- [ ] **Single source of truth**: each meaning lives in one place; other places link to it.
- [ ] **No cache of cheap lookups**: leave `package.json` scripts, the directory tree and `--help` output to the
      environment; write down what no file reveals (a convention, a reason, a gotcha).
- [ ] **No no-ops**: delete a whole sentence the model would obey by default.
- [ ] **Pruning**: remove stale lines when the behaviour changes; superseded meaning goes to `.old/`
      ([[repo/history]]).
- [ ] **Zero-exception rules** are enforced by a hook or a CLI exiting 2, not by prose alone ([[repo/hooks]]).

## Size

| File | Limit |
| --- | --- |
| `AGENTS.md` | As short as possible; well under 200 lines |
| Scope `SKILL.md` index | 2-4 lines of scope, then the routing table |
| Doc page | One subject, about 200 lines at most; split by subject beyond that |
| Runtime skill `SKILL.md` | The procedure only; facts live in linked docs |

The page format (frontmatter, wikilinks, References, Covers) is in [[repo/documentation]].

## References

- [i-have-adhd](https://raw.githubusercontent.com/ayghri/i-have-adhd/main/skills/i-have-adhd/SKILL.md)
- [stop-slop](https://raw.githubusercontent.com/hardikpandya/stop-slop/main/SKILL.md)
- [Writing for agents](https://raw.githubusercontent.com/mattpocock/skills/main/skills/productivity/writing-for-agents/SKILL.md)
