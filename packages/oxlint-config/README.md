# Oxlint config

Add `@yaai/oxlint-config` as a workspace dependency, then extend its JSON file in `.oxlintrc.json`.

```json
{ "extends": ["./node_modules/@yaai/oxlint-config/config.json"] }
```

Run `bin/lint` from the checkout for findings with rule references. See [lint rules](../../docs/repo/lint-rules.md).
