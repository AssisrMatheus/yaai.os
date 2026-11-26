// spec: docs/repo/hooks.md
import { spawnSync } from 'node:child_process';
import { existsSync, statSync, readFileSync, writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { isAbsolute, relative, resolve, join, extname, dirname } from 'node:path';
import { createHash } from 'node:crypto';
import { physical, vendor } from './policy.ts';

export function editPaths(input: Record<string, unknown>, patch = false): string[] {
  if (patch && typeof input.command === 'string') {
    return [...input.command.matchAll(/^\*\*\* (?:Add File|Update File|Move to|Delete File): (.+)$/gm)].map((m) =>
      m[1].trim()
    );
  }
  return [
    ...new Set(
      [
        input.file_path,
        input.notebook_path,
        input.TargetFile,
        input.path,
        ...(Array.isArray(input.paths) ? input.paths : [])
      ].filter((p): p is string => typeof p === 'string')
    )
  ];
}

function run(executable: string, args: string[], root: string) {
  return spawnSync(executable, args, { cwd: root, encoding: 'utf8' });
}

export function docLinks(root: string, path?: string) {
  const bin = join(root, 'bin/doc-links');
  if (!existsSync(bin)) return;
  const args = ['check', ...(path ? [path] : [])];
  const result = run(bin, args, root);
  if (result.status !== null && result.status !== 0)
    return 'Fix the links reported by bin/doc-links ' + args.join(' ').replace(/[\r\n]/g, ' ') + '.';
}

function rustEdition(root: string, file: string) {
  let dir = dirname(file);
  while (dir.startsWith(root)) {
    const manifest = join(dir, 'Cargo.toml');
    if (existsSync(manifest)) {
      const match = readFileSync(manifest, 'utf8').match(/^\s*edition\s*=\s*["'](2015|2018|2021|2024)["']/m);
      if (match) return match[1];
    }
    if (dir === root) break;
    dir = dirname(dir);
  }
  return '2024';
}

export function postEdit(root: string, cwd: string, paths: string[]) {
  let failure: string | undefined;
  for (const path of paths) {
    const file = physical(resolve(cwd, path));
    const name = relative(root, file);
    if (
      name.startsWith('../') ||
      isAbsolute(name) ||
      vendor(root, cwd, path) ||
      !existsSync(file) ||
      !statSync(file).isFile()
    )
      continue;
    let formatter: string | undefined;
    let args: string[] = [];
    if (extname(file) === '.rs' && Bun.which('rustfmt')) {
      const probe = run('rustfmt', ['--version'], root);
      if (probe.status === 0) {
        formatter = 'rustfmt';
        args = ['--config', 'skip_children=true', '--edition', rustEdition(root, file), file];
      }
    } else if (/\.(?:[cm]?[jt]sx?|jsonc?)$/.test(file)) {
      const local = join(root, 'node_modules/.bin/oxfmt');
      formatter = Bun.which(local) ?? Bun.which('oxfmt') ?? undefined;
      args = ['--write', file];
    }
    if (formatter) {
      const result = run(formatter, args, root);
      if (result.status !== null && result.status !== 0)
        failure ??= 'Fix the formatting error in ' + name.replace(/[\r\n]/g, ' ') + '.';
    }
    if (name.startsWith('docs/')) {
      const issue = docLinks(root, name);
      failure ??= issue;
    }
  }
  return failure;
}

function baseline(root: string, session?: string) {
  if (!session) return;
  const key = createHash('sha256').update(session).digest('hex');
  const result = run('git', ['rev-parse', '--git-path', 'yaai-hooks/' + key], root);
  if (result.status === 0) return resolve(root, result.stdout.trim());
}

export function rememberHead(root: string, session?: string) {
  const file = baseline(root, session);
  if (!file || existsSync(file)) return;
  const head = run('git', ['rev-parse', '--verify', 'HEAD'], root);
  mkdirSync(dirname(file), { recursive: true });
  try {
    writeFileSync(file, head.status === 0 ? head.stdout.trim() : 'unborn', { flag: 'wx' });
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== 'EEXIST') throw error;
  }
}

const gatePaths = [
  'docs/',
  'bin/',
  '.husky/',
  'Makefile',
  'Dockerfile',
  '*.rs',
  '*.ts',
  '*.tsx',
  '*.mts',
  '*.cts',
  '*.js',
  '*.jsx',
  '*.mjs',
  '*.cjs',
  '*.sh',
  '*.py',
  '*.c',
  '*.h',
  '*.cpp',
  '*.hpp',
  '*.md',
  '*.toml',
  '*.yml',
  '*.yaml',
  '*.json',
  '*.jsonc',
  '*.css',
  '*.html',
  '*.sql'
];

export function stop(root: string, session?: string) {
  if (!existsSync(join(root, 'bin/doc-links'))) return;
  const file = baseline(root, session);
  const head = file && existsSync(file) ? readFileSync(file, 'utf8') : undefined;
  const changes = run('git', ['status', '--porcelain=v1', '-z', '--untracked-files=all', '--', ...gatePaths], root);
  let changed = changes.status === 0 && Boolean(changes.stdout);
  if (!changed && head) {
    changed =
      head === 'unborn'
        ? run('git', ['rev-parse', '--verify', 'HEAD'], root).status === 0
        : run('git', ['diff', '--quiet', head, 'HEAD', '--', ...gatePaths], root).status === 1;
  }
  const reason = changed ? docLinks(root) : undefined;
  if (!reason && file) rmSync(file, { force: true });
  return reason;
}
