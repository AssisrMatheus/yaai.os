// spec: docs/repo/hooks.md
// spec: docs/repo/rtk.md
// spec: docs/repo/graphify.md
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { editPaths, postEdit, rememberHead, stop } from './checks.ts';
import { shellPolicy, vendor, vendorMessage } from './policy.ts';

export const root = resolve(import.meta.dir, '../..');
type Input = Record<string, unknown>;
type Event = {
  hook_event_name?: string;
  tool_name?: string;
  tool_input?: Input;
  cwd?: string;
  session_id?: string;
  toolCall?: { name: string; args: Input };
  conversationId?: string;
  error?: string;
  terminationReason?: string;
  fullyIdle?: boolean;
  source?: string;
};
type Output = {
  reason?: string;
  hookSpecificOutput?: { updatedInput?: Input; additionalContext?: string; [key: string]: unknown };
};

function optional(command: string, args: string[], event: Event): Output {
  const env = { ...process.env, CLAUDE_PROJECT_DIR: root };
  Reflect.deleteProperty(env, 'RTK_REWRITE_HOST');
  const result = spawnSync(command, args, {
    cwd: root,
    input: JSON.stringify(event),
    timeout: 3000,
    encoding: 'utf8',
    env
  });
  if (result.status !== 0 || !result.stdout) return {};
  try {
    return JSON.parse(result.stdout) ?? {};
  } catch {
    return {};
  }
}

const aliases: Record<string, string> = {
  run_command: 'Bash',
  view_file: 'Read',
  list_dir: 'Glob',
  grep_search: 'Grep',
  find_by_name: 'Glob',
  bash: 'Bash',
  read: 'Read',
  glob: 'Glob',
  grep: 'Grep',
  find: 'Glob',
  write: 'Write',
  edit: 'Edit',
  write_to_file: 'Write',
  replace_file_content: 'Edit',
  multi_replace_file_content: 'Edit'
};
const editors = new Set(['Edit', 'Write', 'MultiEdit', 'NotebookEdit', 'apply_patch']);
const wholeDiff = /\bgit\s+(?:[^;&|]*\s)?(?:diff|show)\b/;

export function evaluate(client: string, mode: string, raw: Event, repo = root) {
  const name = raw.toolCall?.name ?? raw.tool_name ?? '';
  const tool = aliases[name] ?? name;
  const input = raw.toolCall?.args ?? raw.tool_input ?? {};
  const cwd = resolve(repo, String(input.Cwd ?? input.workdir ?? raw.cwd ?? '.'));
  const command = String(input.CommandLine ?? input.command ?? '');
  const event = {
    ...raw,
    cwd,
    tool_name: tool,
    session_id: raw.session_id ?? raw.conversationId,
    tool_input: {
      ...input,
      command,
      file_path: input.file_path ?? input.AbsolutePath ?? input.path,
      pattern: input.pattern ?? input.Query
    }
  };
  const graph = existsSync(resolve(repo, 'graphify-out/graph.json'));
  let reason: string | undefined;
  let context: string | undefined;
  let rewrite: string | undefined;
  let output: Output = {};
  if (mode === 'start' || (mode === 'pre' && (tool === 'Bash' || editors.has(tool)))) {
    try {
      if (existsSync(resolve(repo, 'bin/doc-links'))) rememberHead(repo, event.session_id);
    } catch {
      // A read-only Git directory still permits policy checks and working-tree validation.
    }
  }
  if (mode === 'start') {
    context =
      'Use AGENTS.md and open docs/SKILL.md for the relevant topic.\n' +
      (graph
        ? 'Graph ready: graphify query "<question>".'
        : 'Build the local graph with GRAPHIFY_BACKEND=none graphify update .');
    if (graph && raw.source === 'compact') optional('graphify', ['hook-guard', 'compact'], event);
  }
  if (mode === 'pre') {
    if (editors.has(tool) && editPaths(input, tool === 'apply_patch').some((p) => vendor(repo, cwd, p)))
      reason = vendorMessage;
    if (tool === 'Bash') reason = shellPolicy(command, repo, cwd);
    if (!reason && graph && ['Read', 'Glob', 'Bash', 'Grep'].includes(tool)) {
      const advice = optional('graphify', ['hook-guard', ['Read', 'Glob'].includes(tool) ? 'read' : 'search'], event);
      context = advice.hookSpecificOutput?.additionalContext ?? advice.reason;
    }
    if (
      !reason &&
      tool === 'Bash' &&
      !wholeDiff.test(command) &&
      !input.run_in_background &&
      !input.async &&
      !input.RunPersistent
    ) {
      const result = optional('rtk', ['hook', 'claude'], event);
      const updated = result.hookSpecificOutput?.updatedInput?.command;
      if (typeof updated === 'string' && updated !== command) rewrite = updated;
    }
  }
  if (mode === 'post' && !raw.error) {
    if (editors.has(tool)) reason = postEdit(repo, cwd, editPaths(input, tool === 'apply_patch'));
    else if (client === 'claude' && !wholeDiff.test(command)) output = optional('rtk', ['hook', 'claude'], raw);
  }
  if (mode === 'stop' && (client !== 'agy' || (raw.fullyIdle && raw.terminationReason === 'model_stop')))
    reason = stop(repo, event.session_id);
  return { reason, context, rewrite, output };
}

export function dispatch(client: string, mode: string, raw: Event) {
  const { reason, context, rewrite, output } = evaluate(client, mode, raw);
  if (client === 'agy') {
    if (mode === 'pre')
      return {
        decision: reason ? 'deny' : 'allow',
        reason: reason ?? context,
        overwrite: rewrite ? { CommandLine: rewrite } : undefined
      };
    if (mode === 'start') return { injectSteps: [{ systemMessage: { systemMessage: context } }] };
    if (mode === 'stop') return { decision: reason ? 'continue' : 'stop', reason };
  }
  if (reason) {
    console.error(reason);
    process.exitCode = 2;
    return;
  }
  if (context || rewrite)
    return {
      hookSpecificOutput: {
        hookEventName: raw.hook_event_name ?? (mode === 'start' ? 'SessionStart' : 'PreToolUse'),
        additionalContext: context,
        permissionDecision: rewrite && client === 'codex' ? 'allow' : undefined,
        updatedInput: rewrite ? { ...raw.tool_input, command: rewrite } : undefined
      }
    };
  return output;
}

if (import.meta.main) {
  try {
    const output = dispatch(process.argv[2], process.argv[3], await Bun.stdin.json());
    if (output) console.log(JSON.stringify(output));
  } catch {
    console.log('{}');
  }
}
