// spec: docs/repo/ci.md#timing-and-caching
import { expect, test } from 'bun:test';
import { relative, resolve, sep } from 'node:path';

const suite = relative(process.cwd(), import.meta.path)
  .split(sep)
  .join('/')
  .replace(/\.test\.ts$/, '');
const root = resolve(process.cwd(), '../checkout');

const [scope = '', workspace, kind, target] = suite.split('/');
const cwd = workspace ? resolve(root, scope, workspace) : root;

/** Select the native runner for the file or Cargo target named by this adapter. */
function command(): string[] {
  if (suite === 'root-hooks') return ['bun', 'test', '.agents/hooks'];
  if (/\.[cm]?[jt]sx?$/.test(suite)) return ['pnpm', 'exec', 'bun', 'test', resolve(root, suite)];
  if (!kind) return ['pnpm', 'run', 'test'];
  const args = ['cargo', 'test', '--locked', '--' + kind];
  if (kind !== 'lib' && kind !== 'doc' && target) args.push(target);
  return args;
}

test(
  suite,
  () => {
    const result = Bun.spawnSync(command(), { cwd, stdout: 'inherit', stderr: 'inherit' });
    expect(result.exitCode).toBe(0);
  },
  1_800_000
);
