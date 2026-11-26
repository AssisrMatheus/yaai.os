// spec: docs/repo/lint-rules.md#diagnostics
import { dirname, resolve, relative, sep } from 'node:path';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { type Rule, messageFor } from '../../bindings.ts';

/** Locate the enclosing pnpm workspace for a source file. */
function workspaceRoot(filename: string) {
  let root = dirname(filename);
  while (!existsSync(resolve(root, 'pnpm-workspace.yaml'))) {
    const parent = dirname(root);
    if (parent === root) return undefined;
    root = parent;
  }
  return root;
}

/** Collect package names owned by other applications in this checkout. */
function appNames(root: string, ownDirectory: string | undefined) {
  const apps = resolve(root, 'apps');
  const names = new Set<string>();
  if (!existsSync(apps)) return names;
  for (const dir of readdirSync(apps, { withFileTypes: true })) {
    const manifest = resolve(apps, dir.name, 'package.json');
    if (dir.name !== ownDirectory && dir.isDirectory() && existsSync(manifest)) {
      const { name } = JSON.parse(readFileSync(manifest, 'utf8'));
      if (typeof name === 'string') names.add(name);
    }
  }
  return names;
}

export const workspaceImports = {
  create(context) {
    const root = workspaceRoot(context.filename);
    if (!root) return {};
    const owner = relative(root, context.filename).split(sep);
    if (owner[0] !== 'packages' && owner[0] !== 'apps') return {};
    const ownDirectory = owner[0] === 'apps' ? owner[1] : undefined;
    const apps = appNames(root, ownDirectory);
    const message = messageFor(
      'oxlint::yaai/workspace-imports',
      'The import crosses an application boundary. Move shared code into a package.'
    );
    const foreign = (source: string) => {
      if (source.startsWith('.') || source.startsWith('/')) {
        const target = relative(root, resolve(dirname(context.filename), source)).split(sep);
        return target[0] === 'apps' && ownDirectory !== target[1];
      }
      return [...apps].some((name) => source === name || source.startsWith(name + '/'));
    };
    return {
      ImportDeclaration(node) {
        if (foreign(node.source.value)) context.report({ node, message });
      },
      ExportNamedDeclaration(node) {
        if (node.source && foreign(node.source.value)) context.report({ node, message });
      },
      ExportAllDeclaration(node) {
        if (foreign(node.source.value)) context.report({ node, message });
      },
      ImportExpression(node) {
        if (node.source.type === 'Literal' && typeof node.source.value === 'string' && foreign(node.source.value))
          context.report({ node, message });
      }
    };
  }
} satisfies Rule;
