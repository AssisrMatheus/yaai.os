---
description: How code keeps secrets, verifies TLS, escapes output, constrains paths and origins, handles untrusted uploads and keeps development tools out of release builds.
tags: [best-practices, security]
---

# Security

Input validation and authorisation at boundaries are in [[best-practices/data-boundaries.rules]]; redaction of logs is in
[[best-practices/logging.rules]].

## Secrets and release builds

### BP-SEC-001: Keep secrets out of version control

Keep credentials, tokens and private keys out of version control; `.gitignore` covers every `.env*` file except the template.
Why: a committed secret is public for the life of the history.
Check: lint

### BP-SEC-002: Ship only public configuration to clients

Only values explicitly marked public reach UI or client code; a secret is never given the public prefix or embedded in a client bundle.
Why: anything shipped to a client is readable by its user.
Check: review

### BP-SEC-003: Mark trusted-only modules

A module holding secrets, credentials or privileged clients is marked so that importing it from client code fails the build.
Why: a wrong import then fails loudly instead of leaking a token.
Check: review

### BP-SEC-004: Strip development tools from release builds

Release builds ship no public source maps, debug bridges, agent automation bridges or development tools; check the built output, not the configuration.
Why: they expose source, internals and control surfaces.
Check: test

## Transport security

### BP-SEC-005: Always verify certificates

Every TLS connection verifies the certificate chain and the host name; nothing sets a process-wide switch that disables verification.
Why: encryption without verification stops nobody in the middle.
Check: lint

### BP-SEC-006: Configure TLS in code, not in connection strings

Set TLS options in code and strip TLS parameters from connection URLs, so a pasted URL can disable TLS only explicitly and can never weaken verification.
Why: some drivers let a URL parameter replace the CA or skip the check.
Check: review

### BP-SEC-007: Drop TLS only on explicit request

Connect without TLS only to a loopback address with no TLS setting, or when the URL disables it explicitly; refuse a server that does not offer TLS otherwise.
Why: an attacker can make a server look like it has no TLS.
Check: test

### BP-SEC-008: Trust a private CA through configuration

Read a private CA from an environment variable per environment (PEM or base64 of it); when set, trust only that CA.
Why: process-start-only trust settings are not read from env files, and environments must not inherit another's CA.
Check: review

### BP-SEC-009: Use one connection helper everywhere

The app, migrations, code generators, seeds and scripts open connections through the same helper.
Why: a script with its own settings is where verification gets turned off.
Check: review

### BP-SEC-010: Test that a wrong certificate is refused

Prove verification with a test that connects using a wrong CA and expects a certificate error.
Why: a connection that succeeds with the wrong CA means the check is off somewhere.
Check: test

### BP-SEC-011: Verify by name, even when connecting by address

Prefer host names; when connecting by IP, verify the certificate against that IP or an explicit server name.
Why: turning verification off to fix an IP mismatch removes all protection.
Check: review

## Injection and output

### BP-SEC-012: Pass arguments, never build command strings

Run subprocesses with argument arrays (`Command::new(...).arg(...)`) and query databases with parameters; never interpolate input into a shell or query string.
Why: interpolation turns data into code.
Check: review

### BP-SEC-013: Escape for the destination

Escape output for where it goes (HTML, attribute, URL, shell, JSON inside HTML); embed structured data in HTML through one helper that escapes `<`.
Why: each destination has its own injection syntax.
Check: review

### BP-SEC-014: Sanitise untrusted markup with an allowlist

Sanitise user- or editor-supplied rich text and SVG with an allowlist before inserting it as markup.
Why: uploaded markup can carry scripts.
Check: review

### BP-SEC-015: Serve uploaded vectors as attachments

When serving user-uploaded SVG through an image service, send it as an attachment with a sandboxing content security policy.
Why: an SVG opened directly runs its scripts in the app's origin.
Check: review

### BP-SEC-016: Constrain paths to their root

Resolve a user-supplied path, canonicalise it and reject it unless it stays under the intended root.
Why: `..` and symlinks otherwise reach any file the process can read.
Check: test

### BP-SEC-017: Bound requests and allow origins explicitly

Bound request and upload sizes, and allow only configured schemes and origins for fetches, proxies and image services; reach local or private addresses only in development.
Why: unbounded input exhausts resources, and open fetchers become server-side request forgery.
Check: review

### BP-SEC-018: Compare secrets in constant time

Verify signatures, tokens and webhook secrets with a constant-time comparison.
Why: an early-exit comparison leaks the secret through timing.
Check: review

### BP-SEC-019: Keep credentials out of URLs and keys

Never put a credential in a query string, a cache key or a log field; carry it in a header or an opaque, uncached path.
Why: URLs and keys are logged, cached and shared.
Check: review

## Access and privacy

### BP-SEC-020: Make access control explicit on every resource

Declare read, create, update and delete access on every collection or resource; never rely on an inherited default.
Why: an inherited default is a security decision nobody made.
Check: review

### BP-SEC-021: Embed third parties without tracking

Embed third-party media through privacy-preserving hosts and load the player only when the user asks (click-to-play), so no third-party cookie or request happens before that.
Why: the product shows no ads and leaks no viewing behaviour (INV-PRODUCT-001).
Check: review

### BP-SEC-022: Load integrations only where used

Load a third-party script or SDK only on the screen that needs it, with the narrowest permissions it accepts.
Why: every integration is code from someone else running with our privileges.
Check: review

### BP-SEC-023: Turn off default data collection in telemetry

Configure telemetry SDKs with personal-data collection explicitly off.
Why: SDK defaults often collect IPs, cookies, headers and query strings.
Check: review

## Covers

- `apps/`, `packages/`, `tooling/` (connection helpers, escaping, uploads and access rules)
- `.gitignore`
