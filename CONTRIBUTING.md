# Contributing

Short version: write the doc first, then the code. Small commits. Let the checks tell you what is wrong.

## Set up

You need macOS, Linux or WSL, with Node 24+, pnpm, bun and Rust.

```sh
pnpm install
```

That also installs the git hooks.

## How a change goes

1. **Find the doc.** Start at `docs/SKILL.md`. It points to the page that covers what you are changing.
2. **Change the doc first.** New behaviour, a new rule or a new screen starts as a page in `docs/`.
3. **Write the code.** It follows the doc and points back at it in a comment, like `spec: docs/repo/hooks.md`.
4. **Run the checks.**

```sh
pnpm lint
pnpm test
```

If the code and a doc disagree, the code is wrong.

## Replacing a rule

Don't delete the old text. Move it to the `.old/` folder next to it. `docs/repo/history.md` shows how.

## Commits

- [Conventional Commits](https://www.conventionalcommits.org): `feat(hooks): ...`, `docs(repo): ...`, `fix(doc-links): ...`
- One scope per commit. Commit often.
- Never use `--no-verify`. If a hook stops you, it says what to fix.

A commit that changes code a doc covers must include that doc. If the doc still holds, say so in the message:

```text
Spec-unchanged: only the file layout changed
```

## Rules that never bend

They live in `docs/invariants/`. If your change breaks one, stop and open a discussion first.

## Code

- Rust first. TypeScript only where Rust does not fit.
- The coding rules are in `docs/best-practices/`.
- Every tool in `tooling/` runs from the command line.
- Never edit anything in `vendor/`.

## Working with an AI agent

Agents read `AGENTS.md` and the same docs you do. Their skills and hooks are in `.agents/`.

## Questions

Open an issue.
