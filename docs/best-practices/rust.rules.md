---
description: Language-level Rust rules for naming, types, traits, macros, documentation and gates; the base every other language page refers to.
tags: [best-practices, rust]
---

# Rust

How Rust is written here. Rust is the default language (ADR 0002); other languages map to these rules in
[[best-practices/typescript.rules]]. Errors are in [[best-practices/rust-errors.rules]], ownership and hot paths in
[[best-practices/rust-ownership.rules]], concurrency in [[best-practices/rust-async.rules]], `unsafe` and FFI in
[[best-practices/rust-unsafe.rules]]. Module shape is in [[best-practices/module-design.rules]].

## Names and types

### BP-RUST-001: Use glossary terms in code

Name every type, field, function, command and diagnostic with the term `docs/glossary.md` defines for the concept.
Why: code, docs and agents then share one meaning per word (INV-DOCS-009).
Check: review

### BP-RUST-002: Give each identity its own newtype

Wrap each kind of identifier or measured quantity in a newtype (`struct TitleId(u64)`), never a bare integer or string.
Why: the compiler then rejects a title id passed where a library id belongs.
Check: review

### BP-RUST-003: Model exclusive states as an enum

Represent mutually exclusive states as enum variants carrying their own data, not as several `bool` or optional fields.
Why: impossible combinations become unrepresentable; `clippy::struct_excessive_bools` and `fn_params_excessive_bools` flag the flag soup.
Check: lint

### BP-RUST-004: Model absence with Option

Use `Option<T>` for a value that may be absent, never a sentinel (`""`, `-1`, `0`, `u64::MAX`).
Why: a sentinel compiles everywhere a real value does and leaks into arithmetic and display.
Check: review

### BP-RUST-005: Match owned enums exhaustively

List every variant of an enum the workspace owns; use no wildcard arm on it (`clippy::wildcard_enum_match_arm`).
Why: a new variant then fails the build at every place that must decide how to handle it.
Check: lint

### BP-RUST-006: Parse external values into an enum before branching

Convert an external string or code into a domain enum with `FromStr` or `TryFrom` at entry, with an explicit fallback for unknown values, then `match` the enum.
Why: unknown values get one decided outcome instead of falling through string comparisons.
Check: review

### BP-RUST-007: Keep content in data

Store per-item values (a title's metadata, a source's settings) in data read at run time, not in an `impl` or `match` arm per item.
Why: adding an item is then a data change, and traits stay about code, not content.
Check: review

### BP-RUST-008: Name thresholds and limits

Give every number in a condition, timeout, retry count, size budget or cache lifetime a named `const` (`MAX_RETRIES`).
Why: the name records the rule and gives it one place to change.
Check: review

### BP-RUST-009: Carry units in types

Express durations as `Duration` and sizes, rates or offsets as unit newtypes instead of integers with the unit in the name.
Why: unit mix-ups become type errors.
Check: review

### BP-RUST-010: Convert with From and TryFrom, not as

Use `From`/`TryFrom` for conversions; a lossy `as` cast needs `#[expect(clippy::cast_possible_truncation, reason = "...")]` (`clippy::pedantic` casts, `trivial_casts`, `trivial_numeric_casts`).
Why: `as` truncates and wraps silently.
Check: lint

### BP-RUST-011: Prefer expressions and inferred locals

Bind the result of `match`/`if` expressions, return expressions directly and annotate a local only when inference cannot (`clippy::needless_return`, `let_and_return`).
Why: less code to read, and types follow the implementation instead of drifting from it.
Check: lint

### BP-RUST-012: Use the language before hand-rolling

Reach for `let`-`else`, `?`, `if let` chains and iterator adaptors before manual loops and flags (`clippy::manual_*` lints).
Why: the standard form is shorter and other readers recognise it.
Check: lint

## Traits and generics

### BP-RUST-013: Implement standard traits before inventing methods

Implement `From`, `TryFrom`, `FromStr`, `Default`, `Display`, `AsRef` and `Iterator` instead of bespoke equivalents (`clippy::should_implement_trait`, `new_without_default`).
Why: callers get the whole standard ecosystem (`?`, `collect`, `parse`) for free.
Check: lint

### BP-RUST-014: Derive the common traits

Derive `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash` and `Ord` where they are meaningful, and make small plain values `Copy` (`missing_debug_implementations`, `clippy::missing_copy_implementations`).
Why: values stay printable, comparable and cheap to pass.
Check: lint

### BP-RUST-015: Share behaviour through a trait

When two types answer the same question, implement one trait and make callers generic over it, instead of copied functions or a flag argument.
Why: the behaviour is named once and each implementation is checked against it.
Check: review

### BP-RUST-016: Dispatch statically by default

Take `impl Trait` or a generic parameter; use `dyn Trait` only for a set open at run time or cold code, and measure it before using it on a hot path.
Why: generics inline; `dyn` calls through a table.
Check: review

### BP-RUST-017: Use an enum for a closed set

When every variant is known inside the crate, model it as an enum and `match`; use a trait when other crates or tests add implementations.
Why: an enum keeps the data plain and the dispatch exhaustive.
Check: review

### BP-RUST-018: Put shared defaults on the trait

Write behaviour all implementations share as a default method, and shared types or values as associated types or consts.
Why: written once, resolved at compile time.
Check: review

## Macros

### BP-RUST-019: Prefer functions, generics and consts to macros

Reach for a function, then a generic, then a `const fn` or `const` table, and only then `macro_rules!`.
Why: macros are harder to read, debug and type-check.
Check: review

### BP-RUST-020: Generate synchronised items from one list

Use a macro when one declaration must produce several items that must agree (an enum, its names and its dispatch table).
Why: the items cannot drift apart and cost nothing at run time.
Check: review

### BP-RUST-021: Derive before hand-writing

Use standard derives first; treat a derive or attribute macro crate, or a `proc-macro` crate of our own, as a dependency ([[best-practices/dependencies.rules]]).
Why: procedural macros add build time and a supply-chain edge.
Check: review

### BP-RUST-022: Keep macros hygienic and documented

Refer to items as `$crate::path`, keep the macro private to its crate unless another crate calls it, and give it rustdoc with an example.
Why: the macro then works from any call site and explains itself.
Check: review

### BP-RUST-023: Leave no unused macro arm

Delete arms nothing expands (`unused_macro_rules`).
Why: dead arms are untested code paths.
Check: lint

### BP-RUST-024: Hold macro output to every rule

Generated items carry docs, hide no allocation, `unwrap` or `unsafe`, and any emitted `unsafe` carries its `// SAFETY:`.
Why: expanded code ships like written code.
Check: lint

### BP-RUST-025: Test every macro arm

Call the generated code of each arm in a test; a macro that builds a table gets a test comparing it to hand-checked values.
Why: a macro's bugs show only in its expansions.
Check: test

## Documentation and comments

### BP-RUST-026: Document every item

Write rustdoc for every item, public and private, including fields, variants, helpers, macros and tests (`missing_docs`, `clippy::missing_docs_in_private_items`).
Why: graphify indexes the docs, so an agent finds code by what it does.
Check: lint

### BP-RUST-027: Document crates and modules

Open each crate and module with `//!` docs stating what it owns (`rustdoc::missing_crate_level_docs`).
Why: a reader learns the module's job before its items.
Check: lint

### BP-RUST-028: State intent, then contract

Start a doc comment with the item's intent, then add units, `# Errors`, `# Panics`, `# Safety` and an example where they apply (`clippy::missing_errors_doc`).
Why: the interface includes what the signature cannot say.
Check: lint

### BP-RUST-029: Keep rustdoc warning-free

`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` passes: links resolve, no bare URLs, no broken code blocks.
Why: the documented interface stays navigable.
Check: lint

### BP-RUST-030: Make examples doctests

Write examples in rustdoc as compiling doctests and run them with `cargo test`.
Why: an example that does not compile misleads every reader.
Check: test

### BP-RUST-031: Cite the governing doc above the item

Put `spec: docs/<path>.md#<slug>`, `INV-<DOMAIN>-NNN` or `ADR NNNN` in the comment right above the governed item ([[repo/doc-links]]).
Why: `bin/doc-links` checks the link both ways (INV-DOCS-007).
Check: lint

### BP-RUST-032: Comment the why

Explain the constraint behind a surprising line (the bug it prevents, the source it mirrors); never restate what the code says.
Why: the reason is the one thing the code cannot show.
Check: review

### BP-RUST-033: Give every TODO an owner

Write `// TODO(<ticket or owner>): <condition>`; a bare TODO or FIXME is rejected.
Why: an ownerless TODO survives for years.
Check: review

### BP-RUST-034: Commit no placeholder panics

Never commit `todo!()` or `unimplemented!()` (`clippy::todo`, `clippy::unimplemented`); unfinished work is a ticket.
Why: a placeholder compiles and fails at run time.
Check: lint

### BP-RUST-035: Mark comments with Better Comments prefixes

Use `// !` for a warning not to change without reading, `// ?` for an open question and `// *` for an important note.
Why: editors highlight them and reviewers can grep them.
Check: review

### BP-RUST-036: Delete dead code

Remove unused items, imports, lifetimes and stale comments (`dead_code`, `unused_imports`, `unused_lifetimes`, `unused_qualifications`).
Why: searches then lead only to live behaviour.
Check: lint

## Lints and gates

### BP-RUST-037: Inherit the workspace lints

Every crate's `Cargo.toml` has `[lints] workspace = true` and adds no local lint table.
Why: one lint policy for the whole workspace, set in the root `Cargo.toml`.
Check: lint

### BP-RUST-038: Keep clippy clean at pedantic

`cargo clippy --workspace --all-targets -- -D warnings` passes with `clippy::all` and `clippy::pedantic` denied plus the chosen restriction lints.
Why: the gate catches what review misses.
Check: lint

### BP-RUST-039: Fix the code, not the lint

Fix a lint finding; when an exception is justified, put `#[expect(lint, reason = "...")]` on the smallest item, never `#[allow]` (`clippy::allow_attributes`, `allow_attributes_without_reason`).
Why: an `expect` that stops firing becomes an error, so stale exceptions disappear.
Check: lint

### BP-RUST-040: Change lint policy only by owner decision

Never relax or remove a lint in the workspace configuration to get a change green; raise it with the owner.
Why: the lint set is a repository decision, not a per-task convenience.
Check: review

### BP-RUST-041: Format with rustfmt

Format with the root `rustfmt.toml`; `cargo fmt --all -- --check` passes.
Why: formatting is never a review topic.
Check: lint

### BP-RUST-042: Use the pinned toolchain

Build with the toolchain in `rust-toolchain.toml` and the workspace `rust-version`; no crate overrides either.
Why: local, hook and CI runs compile with the same compiler.
Check: review

### BP-RUST-043: Run the same gates everywhere

Run `cargo fmt --check`, `cargo check --workspace --all-targets`, clippy, `cargo doc` and `cargo test --workspace` locally before handing work over, the same set the hooks and CI run.
Why: a gate that only CI runs is discovered late.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (Rust sources and `Cargo.toml` files)
- `Cargo.toml`
- `rustfmt.toml`
- `clippy.toml`
- `rust-toolchain.toml`

## References

- [Codebase design](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/codebase-design/SKILL.md)
- [Domain modeling](https://raw.githubusercontent.com/mattpocock/skills/main/skills/engineering/domain-modeling/SKILL.md)
