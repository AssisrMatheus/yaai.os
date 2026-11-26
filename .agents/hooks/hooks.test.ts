// spec: docs/repo/hooks.md
import { afterEach, expect, test } from 'bun:test';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { docLinks, editPaths, postEdit, stop } from './checks.ts';
import { shellPolicy } from './policy.ts';
import { evaluate, root } from './runtime.ts';

const dirs: string[] = [];
/** Create an isolated repository directory for a hook test. */
function fixture() {
  const dir = mkdtempSync(join(tmpdir(), 'yaai-hooks-'));
  dirs.push(dir);
  return dir;
}
afterEach(() => {
  for (const dir of dirs.splice(0)) rmSync(dir, { recursive: true, force: true });
});

test('shell policy accepts literal reads and supported commands', () => {
  for (const cmd of [
    'cp vendor/source .',
    'cat vendor/file',
    'pnpm exec oxfmt file.ts',
    'bun test test.ts',
    "echo 'npm install; yarn'",
    "rg -- '--no-verify'"
  ]) {
    expect(shellPolicy(cmd, root, root)).toBeUndefined();
  }
});
test('shell policy checks compound commands, wrappers and vendor destinations', () => {
  for (const cmd of [
    'FOO=1 npm i',
    'false || corepack yarn',
    'rtk init',
    'graphify codex install',
    'git commit -n',
    'git push --no-verify',
    'node app.mts',
    'pnpm exec tsx app.ts',
    'bash -c "npm test"',
    'echo x >vendor/a',
    'cd vendor && touch a',
    'cp a ./vendor/b',
    'mv vendor/a b'
  ]) {
    expect(shellPolicy(cmd, root, root)).toBeDefined();
  }
});
test('both sides of patch moves and symlinked vendor directories are protected', () => {
  const dir = fixture();
  mkdirSync(join(dir, 'vendor'));
  symlinkSync('vendor', join(dir, 'ref'));
  const input = { command: '*** Update File: a\n*** Move to: ref/new.ts' };
  expect(editPaths(input, true)).toEqual(['a', 'ref/new.ts']);
  expect(evaluate('codex', 'pre', { tool_name: 'apply_patch', tool_input: input }, dir).reason).toContain('vendor/');
  expect(
    evaluate('agy', 'pre', { toolCall: { name: 'write_to_file', args: { TargetFile: 'ref/new.ts' } } }, dir).reason
  ).toContain('vendor/');
});
test('doc checks are optional, run on edited doc and dirty tree, and surface a failing validator', () => {
  const dir = fixture();
  mkdirSync(join(dir, 'docs'));
  mkdirSync(join(dir, 'bin'));
  writeFileSync(join(dir, 'docs/a.md'), '# A');
  expect(postEdit(dir, dir, ['docs/a.md'])).toBeUndefined();
  const checker = join(dir, 'bin/doc-links');
  writeFileSync(checker, '#!/bin/sh\nprintf "%s\\n" "$@" > ran\nexit 1\n', { mode: 0o755 });
  expect(postEdit(dir, dir, ['docs/a.md'])).toContain('docs/a.md');
  expect(readFileSync(join(dir, 'ran'), 'utf8')).toBe('check\ndocs/a.md\n');
  expect(spawnSync('git', ['init', '-q'], { cwd: dir }).status).toBe(0);
  expect(stop(dir)).toContain('bin/doc-links check');
  expect(readFileSync(join(dir, 'ran'), 'utf8')).toBe('check\n');
  expect(docLinks(dir)).toBeDefined();
});
test('installed project formatter receives the edited path and reports failure', () => {
  const dir = fixture();
  mkdirSync(join(dir, 'node_modules/.bin'), { recursive: true });
  writeFileSync(join(dir, 'node_modules/.bin/oxfmt'), '#!/bin/sh\nprintf "%s\\n" "$@" > args\nexit 1\n', {
    mode: 0o755
  });
  writeFileSync(join(dir, 'file.ts'), 'let x=1');
  expect(postEdit(dir, dir, ['file.ts'])).toContain('file.ts');
  expect(readFileSync(join(dir, 'args'), 'utf8')).toBe('--write\n' + join(dir, 'file.ts') + '\n');
});
test('native hook commands work from a subdirectory and preserve failure protocols', () => {
  for (const [client, path] of [
    ['codex', '.codex/hooks.json'],
    ['agy', '.agents/hooks.json'],
    ['claude', '.claude/settings.json']
  ] as const) {
    const config = JSON.parse(readFileSync(join(root, path), 'utf8'));
    const event =
      client === 'agy'
        ? { workspacePaths: [root], toolCall: { name: 'run_command', args: { CommandLine: 'yarn' } } }
        : { tool_name: 'Bash', tool_input: { command: 'yarn' } };
    const h = (config.hooks ?? config['yaai-os']).PreToolUse[0].hooks[0];
    const r = spawnSync('sh', ['-c', h.command], {
      cwd: join(root, 'bin'),
      env: { ...process.env, CLAUDE_PROJECT_DIR: root },
      encoding: 'utf8',
      input: JSON.stringify(event)
    });
    if (client === 'agy') {
      expect(r.status).toBe(0);
      expect(JSON.parse(r.stdout).decision).toBe('deny');
    } else {
      expect(r.status).toBe(2);
      expect(r.stderr.trim().split('\n')).toHaveLength(1);
    }
  }
});
test('malformed event and absent optional executables pass', () => {
  const bun = process.execPath;
  for (const input of ['{', '{"tool_name":"Bash","tool_input":{"command":"git status"}}']) {
    const r = spawnSync(bun, [join(root, '.agents/hooks/runtime.ts'), 'claude', 'pre'], {
      input,
      env: { PATH: '' },
      encoding: 'utf8'
    });
    expect(r.status).toBe(0);
    expect(JSON.parse(r.stdout)).toEqual({});
  }
});
const pick = (args: string[]) => spawnSync(join(root, 'bin/agent-pick'), args, { encoding: 'utf8' });
test('picker rejects invalid input and ignores duplicate families', () => {
  expect(pick([]).status).toBe(2);
  expect(pick(['unsupported']).stdout).toBe('');
  expect(pick(['claude', 'claude']).stdout.trim()).toBe('claude');
  expect(pick(['codex', 'antigravity']).stderr).toContain('low/medium');
});

test('unrelated output files do not trigger the stop gate', () => {
  const dir = fixture();
  spawnSync('git', ['init', '-q'], { cwd: dir });
  mkdirSync(join(dir, 'bin'));
  writeFileSync(join(dir, 'bin/doc-links'), '#!/bin/sh\nexit 1\n', { mode: 0o755 });
  writeFileSync(join(dir, '.git/info/exclude'), '/bin/doc-links\n');
  writeFileSync(join(dir, 'photo.png'), 'image');
  expect(stop(dir)).toBeUndefined();
  writeFileSync(join(dir, 'main.rs'), 'fn main() {}');
  expect(stop(dir)).toContain('bin/doc-links check');
});
