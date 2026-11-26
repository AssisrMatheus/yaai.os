---
name: turborepo
description: Configure and troubleshoot Turborepo in this monorepo. Use when working with turbo.json, task pipelines, caching, Remote Cache, the turbo CLI, filtering, environment variables, package boundaries, or CI workflows.
tags: [repo, turborepo, tasks, cache]
---

# Turborepo

The complete Turborepo documentation ships inside the installed `turbo` package. Do not rely on this skill for
framework guidance. Always read the bundled docs, which match the installed version exactly.

Start with:

```text
node_modules/turbo/docs/README.md
```

Use that task index to choose the smallest relevant documentation page. Read it before changing Turborepo
configuration, package scripts, or CI workflows. `pnpm exec turbo docs "<query>"` searches the same docs.

If the package manager uses a non-flat `node_modules` layout or a workspace link, resolve the package location first:

```sh
node -p "require.resolve('turbo/package.json')"
```

Then read `docs/README.md` relative to the resolved package directory.

## Which file to read

| You want to                                              | Read                     |
| -------------------------------------------------------- | ------------------------ |
| Know which tasks exist and how they depend on each other | [[repo/turborepo/tasks]] |
| Know where workspaces live and what each kind may import | [[repo/structure]]       |

## Upstream

This page is the `turborepo` skill published by Vercel, with the frontmatter `tags` and the section above added.
Refresh it with:

```sh
gh api repos/vercel/turborepo/contents/skills/turborepo/SKILL.md -q .content | base64 -d
```

## References

- <https://github.com/vercel/turborepo/blob/main/skills/turborepo/SKILL.md>
- <https://turborepo.com/llms.txt>
