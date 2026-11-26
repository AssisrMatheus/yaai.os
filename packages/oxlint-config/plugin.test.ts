// spec: docs/repo/lint-rules.md#diagnostics
import { test, expect } from 'bun:test';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, symlinkSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

test.each([
  [
    'boolean-state',
    "export type State={kind:'idle'|'playing'};",
    'export type State={a:boolean;b:boolean;c:boolean;d:boolean};',
    'docs/best-practices/typescript.rules.md#BP-TS-003'
  ],
  [
    'env-template',
    'export const key = process.env.OK;',
    'export const key = process.env.MISSING;',
    'docs/best-practices/configuration.rules.md#BP-CONF-001'
  ],
  [
    'task-env',
    'export const key = Bun.env.OK;',
    'export const key = Bun.env.MISSING;',
    'docs/best-practices/configuration.rules.md#BP-CONF-005'
  ],
  [
    'workspace-imports',
    "import type { A } from './local'; export type T = A;",
    "import type { A } from '../../apps/player/state'; export type T = A;",
    'docs/best-practices/code-organization.rules.md#BP-ORG-010'
  ],
  [
    'suppression-reason',
    'export const n = 1;',
    '/* oxlint-disable eslint/no-console */\nexport const n = 1;',
    'docs/best-practices/typescript.rules.md#BP-TS-081'
  ],
  [
    'transition-properties',
    "export const s = {transition: 'opacity 100ms'};",
    "export const s = {transition: 'all 100ms'};",
    'docs/best-practices/motion.rules.md#BP-MOTION-007'
  ],
  [
    'translated-copy',
    'export const v = <span>{message}</span>;',
    'export const v = <span>Welcome</span>;',
    'docs/best-practices/i18n.rules.md#BP-I18N-001'
  ],
  [
    'visual-token',
    'export const v = <div style={{color:token}} />;',
    "export const v = <div style={{color:'#fff'}} />;",
    'docs/best-practices/design-tokens.rules.md#BP-TOKEN-004'
  ],
  [
    'verify-tls',
    'export const tls = {rejectUnauthorized:true};',
    'export const tls = {rejectUnauthorized:false};',
    'docs/best-practices/security.rules.md#BP-SEC-005'
  ],
  [
    'button-type',
    'export const v = <button type="button" aria-label={message} />;',
    'export const v = <button aria-label={message} />;',
    'docs/best-practices/ui.rules.md#BP-UI-012'
  ],
  [
    'no-enum',
    "export type Mode = 'quiet';",
    'export enum Mode { Quiet }',
    'docs/best-practices/typescript.rules.md#BP-TS-066'
  ]
])('%s accepts valid code and cites the governing rule', (name, valid, invalid, reference) => {
  const config = fileURLToPath(new URL('./config.json', import.meta.url));
  const directory = mkdtempSync(join(tmpdir(), 'yaai-oxlint-'));
  try {
    expect(name).toBeTruthy();
    const workspace = join(directory, 'packages', 'fixture');
    mkdirSync(workspace, { recursive: true });
    writeFileSync(join(workspace, 'package.json'), '{}');
    writeFileSync(join(directory, 'pnpm-workspace.yaml'), "packages: ['packages/*','apps/*']");
    writeFileSync(join(workspace, '.env.example'), 'OK=');
    writeFileSync(join(directory, 'turbo.json'), JSON.stringify({ globalEnv: ['OK'] }));
    const modules = join(workspace, 'node_modules', '@yaai');
    mkdirSync(modules, { recursive: true });
    symlinkSync(dirname(config), join(modules, 'oxlint-config'), 'dir');
    const consumer = join(workspace, '.oxlintrc.json');
    writeFileSync(consumer, JSON.stringify({ extends: ['./node_modules/@yaai/oxlint-config/config.json'] }));
    const file = join(workspace, 'input.tsx');
    const check = (source: string) => {
      writeFileSync(file, source);
      return spawnSync('oxlint', ['-c', consumer, '--format', 'json', file], { cwd: directory, encoding: 'utf8' });
    };
    const clean = check(valid);
    expect(clean.error).toBeUndefined();
    expect(clean.status).toBe(0);
    const result = check(invalid);
    expect(result.status).toBe(1);
    expect(result.stdout).toContain(reference);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
