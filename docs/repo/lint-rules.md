---
description: Bind best-practice rules to diagnostics and run the repository lint command.
tags: [repo, lint]
---

# Lint rules

## Bindings

Record each diagnostic name against its governing rule in `packages/lints/bindings.json`. One catalog handles
many-to-many bindings without changing the grammar of rule pages. Keys are diagnostic names, and values identify
the page and stable heading ID. Prefer a stock lint from the pinned tool. Use a custom syntax check only when the
installed tool lacks the check and valid and invalid inputs are distinguishable. Keep decisions about intent,
architecture, measured performance and product-specific semantics under review.

Use oxlint for JS and TS syntax, Clippy for typed Rust, and the compiler for native diagnostics. Keep existing
formatters, doc-links, cargo-machete and cargo-deny as the checks for their own inputs. The dependency gate permits
MIT, Apache-2.0 and Unicode-3.0 for third-party crates. `packages/lints/cargo-deny.toml` records that policy.
The custom Rust syntax trade-off is recorded in [[adr/0014-stable-syntax-lints.adr]].

The consistency check resolves TypeScript's “Same as” entries against the Rust heading's classification. Bind the
TypeScript ID to its own applicable check. A rule can contain both a detectable pattern and a judgement call:
for example, Clippy detects excessive flags while review decides which states actually exclude each other.

## Order of preference

Decided by the owner on 2026-11-26. Apply the steps in order and stop at the first that fits.

1. **An existing, supported rule before a rule of our own.** This includes the default rule sets that ship for a
   language, framework or library: import the set, then add ours on top. Never re-create a stock rule, and never
   hand-pick stock rules in place of importing the defaults they belong to.
2. **Clippy before oxlint.** Rust is linted by Clippy and the compiler. Oxlint lints what is not Rust, and it is the
   linter for everything it can lint: no second JavaScript or TypeScript linter is added beside it.
3. **A custom rule last**, in the one package for its ecosystem (`packages/lints/` for Rust, `packages/oxlint-config/`
   for oxlint), under the scope folder of its rule page. Its catalog entry names the stock rules that were considered
   and why none fits.
4. **Our own linter only where neither tool can run.** Clippy sees Rust only, and oxlint does not load custom file
   formats or parsers, so Markdown structure and links across files stay in `bin/doc-links`
   ([[repo/doc-links]]). Revisit when oxlint supports custom file formats. Formatting is separate: oxfmt formats
   Markdown, JSON, YAML and TOML as well as code.

## Diagnostics

Run `bin/lint` for agent-facing output. The runner appends the binding to stock diagnostic text. A custom finding
uses `<what is wrong>. <what to do>. Rule: docs/best-practices/<page>.rules.md#<ID>`.
A diagnostic from a stock lint group uses the group's governing rule when it has no more specific binding.

## Add a binding

1. Read the rule body and check the installed tool's rule list and options.
2. Record the lint and the rule reference in the catalog. Enable it in the shared config or Cargo workspace.
3. For custom rules, add a scope module and exercise both a valid file and an invalid file.
4. Run the lint on an invalid example and check its rule reference, then run the repository checks.

## Covers

- `packages/lints/`
- `packages/oxlint-config/`
- `tooling/lint/`
- `Cargo.toml`
- `clippy.toml`
- `.oxlintrc.json`
- `tsconfig.json`
- `bin/lint`

## References

- https://oxc.rs/llms.txt
- https://rust-lang.github.io/rust-clippy/master/
- https://github.com/trailofbits/dylint
