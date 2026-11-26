---
name: yaai-repo
description: How the yaai.os monorepo works, for agents changing it. Use when following the working loop, delegating to sub-agents, writing or superseding docs, adding a workspace, hook or skill, committing, or using rtk, graphify, psychoscript or Turborepo.
tags: [repo, index]
---

# How this monorepo works

The rules of the repository itself: how agents work in it, how its documentation is written and checked, and how its
tooling is wired. Product behaviour lives in the other scopes.

## Which file to read

| You want to | Read |
| --- | --- |
| Start a task: the docs-first loop, plans, what "done" means | [[repo/workflow]] |
| Split work across sub-agents, pick a provider and model, write a brief | [[repo/delegation]] |
| Add or edit a page or a scope; frontmatter, wikilinks, indexes, references | [[repo/documentation]] |
| Know which file ending marks a doc kind and what each kind must contain | [[repo/doc-kinds]] |
| Replace a rule or page and keep the old text (`.old/`) | [[repo/history]] |
| Write a README (humans) or an AGENTS.md, skill or doc (agents) | [[repo/writing]] |
| Cite a doc from code, list covered code, run `doc-links` | [[repo/doc-links]] |
| Know what lives in `apps`, `packages`, `tooling`, `bin`, `vendor`; add a workspace | [[repo/structure]] |
| Check supported systems and prerequisites | [[repo/platforms]] |
| See how each agent runtime finds skills, hooks and roles; add one | [[repo/agent-runtimes]] |
| Understand why a hook stopped you, or add a hook | [[repo/hooks]] |
| Read condensed command output, or change the rtk pipeline | [[repo/rtk]] |
| Ask where something is or what calls it, before grepping | [[repo/graphify]] |
| Batch tool calls in one program (code mode) | [[repo/psychoscript]] |
| Write a commit message, or fix a rejected commit | [[repo/commits]] |
| Change Turborepo tasks, caching or `turbo.json` | [[repo/turborepo/SKILL]] |
