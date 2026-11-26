---
description: What psychoscript is, why it is the repository's code mode, its honest status, the planned host, and the native fallbacks used meanwhile.
tags: [repo, agents, psychoscript, code-mode]
---

# psychoscript: the repository's code mode

psychoscript is a confined JavaScript runtime for programs that call schema-described tools. A code mode lets an agent
write one program that batches several tool calls and returns one result, instead of many sequential tool turns. The
owner decided that psychoscript is this repository's code mode for every agent runtime (INV-AGENTS-004, ADR 0010).

## Status

Upstream ships the language: compiler, VM, snapshots, runtime and host bindings. The JS package exports a Promise API
and an optional Effect API (Bun >= 1.4 or Node >= 24) and has no `bin`. The Rust crate exports `Runtime`,
`HostFunction`, `HostValue` and `Limits`. Its bench and conformance runners are not agent hosts. No supported
ready-made CLI or MCP server connects the interpreter to a provider's tool registry.

So psychoscript batching is pending a host. No dependency and no MCP entry for it exists in this repository, and
none is faked. Until the host exists, use the [[repo/psychoscript#Native fallbacks]] and keep calls independent.

## Planned host: `tooling/psychoscript-host`

Status: Planned. Another agent is building it. When it exists, flip this section to built and add it to `## Covers`.

- One small Rust CLI with a stdio MCP transport, plus a one-shot stdin JSON mode that calls the same evaluator for
  shell-only clients.
- One tool, `execute({source})`; its description lists the available function signatures.
- A fixed registry first: local graph queries ([[repo/graphify]]) and `bin/doc-links check` ([[repo/doc-links]]).
- Each allowed schema binds to a `HostFunction`. The host validates arguments, dispatches a fixed argv or an
  authorized upstream MCP method, validates structured results, caps runtime and output, and propagates cancellation.
- Programs use async host functions and return one JSON value. The VM has no implicit filesystem, network or process
  access, and no broad shell or `eval` global.
- Exit codes follow [[best-practices/tooling-cli.rules]]: 0 success, 1 host failure, 2 invalid input the caller must fix.
- Registration, once built: `.mcp.json` (shared by Antigravity, Pi and OMP) and `.codex/config.toml`. T3 inherits the
  provider's config.
- A generic external MCP server cannot call the parent client's private tool registry. The host exposes explicit
  backend tools, or a provider extension forwards to the provider's authorized dispatcher and keeps native approvals
  and hooks. Test parallel independent reads, ordered dependent calls, schema errors and cancellation before
  documenting parity with native tools.

## Native fallbacks

These are separate runtimes from psychoscript. Use them for batching today.

| Runtime | Setting | Where |
| --- | --- | --- |
| Codex | `[features.code_mode] enabled = true` and `code_mode_only = true` | `.codex/config.toml` |
| Pi | `+codemode` tool with `codemode.mode = "only"` (QuickJS) | `.pi/settings.json` |
| Oh My Pi | `eval.js: true` | `.omp/config.yml` |
| Claude Code | Opt in with `CLAUDE_CODE_EXPERIMENTAL_CODE_MODE=1` in your terminal or user scope; no project setting exists, and none for its SDK inside T3 | user scope |
| Antigravity | No verified project setting | none |

## Covers

- `.codex/config.toml`
- `.pi/settings.json`
- `.omp/config.yml`

## References

- [Claude Code code mode](https://code.claude.com/docs/en/code-mode.md)
- [Pi codemode](https://docs.pi.dev/latest/codemode)
- [Oh My Pi eval tool](https://github.com/can1357/oh-my-pi/blob/main/docs/tools/eval.md)
- [Oh My Pi README](https://github.com/can1357/oh-my-pi/blob/main/README.md)
- [psychoscript README](https://github.com/anomalyco/psychoscript/blob/dev/README.md)
- [psychoscript AGENTS.md](https://github.com/anomalyco/psychoscript/blob/dev/AGENTS.md)
- [psychoscript Rust README](https://github.com/anomalyco/psychoscript/blob/dev/rust/README.md)
- [psychoscript package.json](https://github.com/anomalyco/psychoscript/blob/dev/package.json)
- [psychoscript Rust workspace](https://github.com/anomalyco/psychoscript/blob/dev/rust/Cargo.toml)
- [psychoscript crate manifest](https://github.com/anomalyco/psychoscript/blob/dev/rust/crates/psychoscript/Cargo.toml)
- [psychoscript interpreter support](https://github.com/anomalyco/psychoscript/blob/dev/interpreter-support.md)
- [psychoscript website README](https://github.com/anomalyco/psychoscript/blob/dev/website/README.md)
