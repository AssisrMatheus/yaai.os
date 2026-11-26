---
name: yaai-code-review
description: Mandatory adversarial review of your own diff against docs/best-practices/, Rust first then TypeScript. Use before reporting done any change that touches code (.rs, .ts, .tsx, shell, hooks, tool config), and when asked to review a diff, a PR or a path.
---

# Code review

Review a change against the rule pages in `docs/best-practices/`. The pages are the rubric and they change; read them
on every review instead of reviewing from memory, and cite every finding by its rule id (`BP-RUST-048`). This skill
holds only the procedure: how to scope, check, score, resolve and report.

Be adversarial: assume the diff is wrong and look for the input, caller or edge case that proves it. Every finding
you report is **blocking**: fix it or justify it before you report the work done.

## 1. Scope the diff

First match wins:

| Invocation | Mode | Scope |
| --- | --- | --- |
| Your own change (the default before reporting done) | `changes` | `git status`, `git diff`, `git diff --staged` |
| A PR number | `pr` | `gh pr diff <n>` and `gh pr view <n> --json title,body,baseRefName` |
| A path | `path` | `git ls-files <path>` |

Done when you hold the list of changed code files and, for each, what the change does.

## 2. Run the deterministic checks first

Run what applies to the changed files, from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm format:check
pnpm lint          # workspace lints, oxlint, bin/doc-links check
pnpm typecheck
pnpm test
```

Fix every failure before reviewing; the review never repeats what these report. Done when every applicable command
exits 0 on the final diff.

## 3. Pick the rule pages

Open `docs/best-practices/SKILL.md` and choose every row whose "You want to" matches something the diff does.

- Rust: always `rust.rules`, `rust-errors.rules` and `module-design.rules`; add `rust-ownership.rules`,
  `rust-async.rules` and `rust-unsafe.rules` when the diff borrows, mutates state, awaits, spawns or uses `unsafe`.
- TypeScript: `typescript.rules`, then each Rust rule its `Shared with Rust` list points at for what the diff does.
- Then the cross-cutting rows the diff touches: services and adapters, a data store, logging, tests, CLIs, hooks,
  configuration, dependencies, UI. A diff that touches a data store also runs the `yaai-data-review` skill.

Read each chosen page in full. Done when every part of the diff is tied to at least one page you have read.

## 4. Read what governs the code

Open the doc each changed item cites (`spec:`, `screen:`, `INV-`, `ADR`) and the invariants of its area
(`docs/invariants/SKILL.md`). Read whole changed files, not only hunks, when a finding depends on context: a caller,
another user of a shared type, whether a value is on a hot path. Done when you know which doc governs each changed
item, or that one is missing.

## 5. Review

Walk every `Check: review` rule of every chosen page against every hunk, Rust files first, then TypeScript, then
shell and config. Take a `Check: lint` or `Check: test` rule only when its lint or test is not configured yet. Also
check that governed code cites its doc and the doc's `Covers` lists the path (INV-DOCS-007), that names come from
`docs/glossary.md` (INV-DOCS-009), and that the change breaks no invariant. Done when every rule of every chosen page
has been held against the diff.

## 6. Score and filter

Score each candidate finding 0 to 100 and drop anything below 75:

- **0**: false positive, or a pre-existing issue on a line the diff did not touch.
- **25**: might be real, unverified, or no rule states it.
- **50**: real but a nit next to the rest of the diff.
- **75**: verified, a rule states it, and it will bite in practice. Report threshold.
- **100**: certain; the rule names this exact case.

In `changes` and `pr` modes, do not flag untouched lines, anything step 2 catches, nits a senior engineer would not
raise, or the intentional point of the change. In `path` mode pre-existing issues count; the check categories still
do not.

## 7. Resolve every finding

For each finding: fix it and rerun step 2, or write a justification saying why the rule does not hold here. A
justified exception in code uses the rule's own suppression form (`#[expect(lint, reason = "...")]`,
`// oxlint-disable-next-line <rule> -- <reason>`). A finding that needs a doc or invariant change is reported to the
user, never worked around. Done when every finding is fixed or justified and step 2 passes again.

## 8. Report

````markdown
### Code review

**Mode:** changes | PR #<n> | path <path>
**Checks:** <each command run> -> <exit code>
**Pages:** <rule pages read>

1. **<short description>** (BP-<AREA>-NNN, confidence <score>): fixed | justified
   `path/to/file.rs:12-18`
   ```rust
   <the flagged code>
   ```
   **Fix:** <the change, or the justification>
   **Why:** <the rule's Why, applied to this case>

**Result:** <n> findings, <n> fixed, <n> justified, 0 open
````

With nothing to report: `No findings against <pages>. Checked: <rule ids that applied>.` One code sample per finding,
no emojis. The work is done only when the report shows 0 open findings.
