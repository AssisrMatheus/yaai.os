---
name: yaai-workflow
description: Docs-first, ADR-driven development loop for yaai.os. Use when starting any task that changes behaviour, rules, screens, or code, or when a request conflicts with an invariant or governing doc.
---

# yaai.os workflow

Follow `docs/repo/workflow.md` for full rules. Keep `docs/` as the source of truth (`INV-DOCS-001`) and treat code as its reproduction.

## Procedure

1. **Route governing docs by task**: open `docs/SKILL.md`, then only the scope `SKILL.md` files for the task (`INV-AGENTS-003`), plus the relevant files under `docs/invariants/SKILL.md` and `docs/glossary.md`. Query `graphify` before raw file scans.
2. **Check for conflicts**: verify the request against every applicable invariant and governing doc. If the request conflicts with or is ambiguous against an invariant or doc, stop immediately and tell the user which `INV-` / `ADR` / doc path conflicts and why.
3. **Plan concisely**: write an extremely concise plan (sacrifice grammar for concision) and end with unresolved questions, if any. Wait for user decisions on open questions before changing rules or docs.
4. **Update docs first** whenever behaviour, a rule, or a screen changes:
   - New hard-to-reverse trade-off -> ADR via `docs/adr/SKILL.md` (`docs/adr/0000-template.adr.md`).
   - Non-negotiable rule -> invariant in `docs/invariants/` (user-approved only).
   - Behaviour or contract -> spec in `docs/specs/` or guide in `docs/repo/`.
   - Screen or subview -> implementation-neutral doc in `docs/screens/`.
   - Superseded text -> move to the scope's `.old/` folder with replacement and commit (`docs/repo/history.md`).
5. **Implement as reproduction**: write code matching the doc and `docs/best-practices/SKILL.md`. Cite the governing doc right above each governed item (`spec:`, `screen:`, `INV-`, `ADR`) and update the doc's `## Covers` (`docs/repo/doc-links.md`).
6. **Verify deterministically first**: run `bin/doc-links check` (when built), formatting, linting, typecheck, and tests for touched scopes. Fix every reported failure before finishing.
7. **Close the loop**: report changed files, exact commands run with observed verdicts, and open questions. When the same mistake repeats, propose a deterministic check (hook, lint rule, test) or a sharper index pointer instead of adding prose.

## Reference docs

- Loop rules and done bar: `docs/repo/workflow.md`
- Invariants: `docs/invariants/SKILL.md`
- ADRs: `docs/adr/SKILL.md`
- Code-doc citations: `docs/repo/doc-links.md`
- Superseded docs (`.old/`): `docs/repo/history.md`
