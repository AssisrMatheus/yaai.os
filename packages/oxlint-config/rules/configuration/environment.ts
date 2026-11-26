// spec: docs/repo/lint-rules.md#bindings
import { dirname, join } from 'node:path';
import { existsSync, readFileSync } from 'node:fs';
import { type Rule, messageFor } from '../../bindings.ts';

/** Find the nearest directory containing the requested workspace file. */
function ancestor(start: string, file: string) {
  let directory = start;
  while (!existsSync(join(directory, file))) {
    const parent = dirname(directory);
    if (parent === directory) return undefined;
    directory = parent;
  }
  return directory;
}

/** Check a literal environment read against its configured declaration. */
export function environmentRule(check: 'template' | 'task'): Rule {
  return {
    create(context) {
      const directory = dirname(context.filename);
      let names: Set<string> | undefined;
      return {
        MemberExpression(node) {
          const env = node.object;
          if (
            env.type !== 'MemberExpression' ||
            env.object.type !== 'Identifier' ||
            !['process', 'Bun'].includes(env.object.name) ||
            env.computed ||
            env.property.type !== 'Identifier' ||
            env.property.name !== 'env'
          )
            return;
          if (node.parent.type === 'AssignmentExpression' && node.parent.left === node && node.parent.operator === '=')
            return;
          const key =
            !node.computed && node.property.type === 'Identifier'
              ? node.property.name
              : node.computed && node.property.type === 'Literal' && typeof node.property.value === 'string'
                ? node.property.value
                : undefined;
          if (!key) return;
          if (!names) {
            if (check === 'template') {
              const pkg = ancestor(directory, 'package.json');
              if (!pkg) return;
              const template = join(pkg, '.env.example');
              const source = existsSync(template) ? readFileSync(template, 'utf8') : '';
              names = new Set([...source.matchAll(/^\s*(?:export\s+)?([A-Za-z_]\w*)\s*=/gm)].map((m) => m[1] ?? ''));
            } else {
              const root = ancestor(directory, 'turbo.json');
              if (!root) return;
              const config = JSON.parse(readFileSync(join(root, 'turbo.json'), 'utf8'));
              const tasks: { env?: string[]; passThroughEnv?: string[] }[] = Object.values(config.tasks ?? {});
              names = new Set([
                ...(config.globalEnv ?? []),
                ...(config.globalPassThroughEnv ?? []),
                ...tasks.flatMap((t) => (t.env ?? []).concat(t.passThroughEnv ?? []))
              ]);
            }
          }
          if (names.has(key)) return;
          const message =
            check === 'template'
              ? messageFor(
                  'oxlint::yaai/env-template',
                  key + ' has no template entry. Document it in the workspace .env.example.'
                )
              : messageFor(
                  'oxlint::yaai/task-env',
                  key + ' has no task declaration. Declare it in the consuming Turborepo task.'
                );
          context.report({ node, message });
        }
      };
    }
  };
}
