// spec: docs/repo/hooks.md
import { existsSync, lstatSync, readlinkSync, realpathSync } from 'node:fs';
import { basename, dirname, join, relative, resolve } from 'node:path';

/** Resolve a path through existing ancestors and symbolic links. */
export function physical(path: string, seen = new Set<string>()): string {
  if (existsSync(path)) return realpathSync(path);
  if (lstatSync(path, { throwIfNoEntry: false })?.isSymbolicLink()) {
    if (seen.has(path)) return path;
    seen.add(path);
    return physical(resolve(dirname(path), readlinkSync(path)), seen);
  }
  const parent = dirname(path);
  return parent === path ? path : join(physical(parent, seen), basename(path));
}

/** Check whether a resolved destination belongs to the vendor tree. */
export function vendor(root: string, cwd: string, path: string) {
  const target = resolve(cwd, path);
  return [relative(root, target), relative(root, physical(target))].some(
    (name) => name === 'vendor' || name.startsWith('vendor/')
  );
}

const tsRunner = (value: string) => ['tsx', 'ts-node', 'ts-node-esm'].includes(value);
const tsFile = (value: string) => /\.(?:[cm]?ts|tsx)$/.test(value);

const vendorMessage = 'Write to a project-owned path outside vendor/.';
export { vendorMessage };

// Inspect literal commands and paths. The native sandbox controls arbitrary program execution.
/** Return the repair for a literal shell command that violates repository policy. */
export function shellPolicy(command: string, root: string, initialDirectory: string): string | undefined {
  let cwd = initialDirectory;
  const tokens = command.match(/(?:\\[\s\S]|"[^"]*"|'[^']*'|[^\s"'\\;&|()<>\x60])+|&&|\|\||>>|[;&|()<>\n\x60]/g) ?? [];
  let words: string[] = [];
  for (const token of [...tokens, ';']) {
    if (!/^(?:[;|&()\n\x60]|&&|\|\|)$/.test(token)) {
      words.push(token.replace(/(["'])(.*?)\1/gs, '$2').replace(/\\(.)/gs, '$1'));
      continue;
    }
    const args = words;
    words = [];
    while (args.length) {
      const [first = '', next] = args;
      if (
        /^\w+=/.test(first) ||
        ['env', 'command', 'exec', 'sudo', 'then', 'do', 'if', 'bypass', 'corepack'].includes(first) ||
        (first === 'rtk' && next !== 'init')
      )
        args.shift();
      else break;
    }
    const exe = basename(args.shift() ?? '');
    if (['sh', 'bash', 'zsh'].includes(exe)) {
      const flag = args.findIndex((arg) => /^-[a-z]*c[a-z]*$/.test(arg));
      const nested = flag >= 0 ? args[flag + 1] : undefined;
      if (nested) {
        const reason = shellPolicy(nested, root, cwd);
        if (reason) return reason;
      }
    }
    if (
      ['npm', 'npx', 'yarn', 'yarnpkg', 'bunx'].includes(exe) ||
      (exe === 'bun' && ['add', 'install', 'i', 'remove', 'update', 'x'].includes(args[0] ?? ''))
    )
      return 'Use pnpm for package management (pnpm exec or pnpm dlx for package executables).';
    const target = args.find((arg) => !arg.startsWith('-')) ?? '';
    const pnpmTarget = ['exec', 'dlx', 'run'].includes(target) ? (args[args.indexOf(target) + 1] ?? '') : target;
    if (
      tsRunner(exe) ||
      (exe === 'deno' && target === 'run') ||
      (exe === 'node' && args.some((a) => tsRunner(a) || tsFile(a))) ||
      (exe === 'pnpm' && (tsRunner(pnpmTarget) || tsFile(pnpmTarget)))
    )
      return 'Run TypeScript with bun <file.ts>.';
    if (exe === 'git' && (args.includes('--no-verify') || (args.includes('commit') && args.includes('-n'))))
      return 'Run Git with verification enabled and fix any reported checks.';
    if (exe === 'rtk' && args.includes('init')) return 'Use the repository rtk pipeline and hooks.';
    if (exe === 'graphify' && args[1] === 'install') return 'Use the repository Graphify skill and hooks.';
    if (exe === 'cd' && args[0]) cwd = resolve(cwd, args[0]);
    for (const [i, arg] of args.entries()) {
      const outputPath = ['-o', '--output', '--output-document', '--output-directory'].includes(arg)
        ? args[i + 1]
        : arg.match(/^(?:--output(?:-document|-directory)?=|of=)(.+)$/)?.[1];
      if (outputPath && vendor(root, cwd, outputPath)) return vendorMessage;
    }
    if (exe === 'git') {
      const at = args.indexOf('-C');
      const directory = args[at + 1];
      if (
        at >= 0 &&
        directory &&
        vendor(root, cwd, directory) &&
        args.some((a) => ['checkout', 'switch', 'restore', 'reset', 'clean', 'apply', 'pull', 'merge'].includes(a))
      )
        return vendorMessage;
    }
    const redirects = [exe, ...args];
    if (
      redirects.some((a, i) => {
        const next = redirects[i + 1];
        return (a === '>' || a === '>>') && next && vendor(root, cwd, next);
      })
    )
      return vendorMessage;
    const paths = args.filter((a) => !a.startsWith('-'));
    const destinations = ['cp', 'install', 'ln'].includes(exe)
      ? [args.includes('-t') ? args[args.indexOf('-t') + 1] : paths.at(-1)]
      : paths;
    if (
      ([
        'mv',
        'cp',
        'install',
        'ln',
        'rm',
        'rmdir',
        'mkdir',
        'touch',
        'tee',
        'chmod',
        'chown',
        'truncate',
        'patch'
      ].includes(exe) ||
        (['sed', 'perl'].includes(exe) && args.some((a) => /^-[^\s]*i/.test(a)))) &&
      destinations.some((a) => a && vendor(root, cwd, a))
    )
      return vendorMessage;
  }
  return undefined;
}
