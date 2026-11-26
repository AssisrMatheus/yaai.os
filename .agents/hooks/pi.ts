// spec: docs/repo/hooks.md
// Pi can run under Node. Run the same Bun entry point as the command-hook clients.
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

type Tool = { toolName?: string; input?: Record<string, unknown>; isError?: boolean; entries?: unknown[] };
type Context = { cwd: string; sessionManager: { getSessionId(): string }; agent?: { kind: string } };
type Message = { customType: string; content: string; display: boolean };
type Extension = {
  on(name: string, callback: (event: Tool, ctx: Context) => unknown): void;
  sendMessage(message: Message, options?: { triggerTurn: boolean }): void;
};
const message = (content: string): Message => ({ customType: 'repository-hooks', content, display: false });
/** Register repository policy handlers with the selected extension runtime. */
export function register(pi: Extension, client: 'omp' | 'pi') {
  const script = fileURLToPath(new URL('./runtime.ts', import.meta.url));
  /** Run the shared command hook and decode its decision. */
  function hook(mode: string, event: Tool, ctx: Context) {
    const raw = {
      session_id: ctx.sessionManager.getSessionId(),
      cwd: ctx.cwd,
      tool_name: event.toolName,
      tool_input: event.input,
      hook_event_name: mode === 'pre' ? 'PreToolUse' : mode === 'post' ? 'PostToolUse' : 'SessionStart'
    };
    const result = spawnSync('bun', [script, client, mode], { input: JSON.stringify(raw), encoding: 'utf8' });
    if (result.status === 2) return { reason: result.stderr.trim() };
    try {
      return JSON.parse(result.stdout ?? '{}').hookSpecificOutput ?? {};
    } catch {
      return {};
    }
  }
  pi.on('session_start', (event, ctx) => {
    const text = hook('start', event, ctx).additionalContext;
    if (text) pi.sendMessage(message(text));
  });
  pi.on('tool_call', (event, ctx) => {
    if (!['bash', 'read', 'grep', 'glob', 'find', 'write', 'edit'].includes(event.toolName ?? '')) return undefined;
    const result = hook('pre', event, ctx);
    if (result.reason) return { block: true, reason: result.reason };
    if (client === 'omp') return { input: result.updatedInput, additionalContext: result.additionalContext };
    if (result.updatedInput && event.input) Object.assign(event.input, result.updatedInput);
    if (result.additionalContext) pi.sendMessage(message(result.additionalContext));
    return undefined;
  });
  pi.on('tool_result', (event, ctx) => {
    if (event.isError || !['write', 'edit'].includes(event.toolName ?? '')) return undefined;
    const { reason } = hook('post', event, ctx);
    if (reason) {
      if (client === 'omp') return { additionalContext: reason };
      pi.sendMessage(message(reason));
    }
    return undefined;
  });
  if (client === 'omp') {
    pi.on('session_stop', (event, ctx) => {
      const { reason } = hook('stop', event, ctx);
      if (reason) return { decision: 'block', reason };
      return undefined;
    });
    pi.on('agent_end', (event, ctx) => {
      if (ctx.agent?.kind !== 'sub') return;
      const { reason } = hook('stop', event, ctx);
      if (reason) pi.sendMessage(message(reason), { triggerTurn: true });
    });
  } else {
    pi.on('agent_before_settle', (event, ctx) => {
      const { reason } = hook('stop', event, ctx);
      if (reason)
        return { entries: [...(event.entries ?? []), { type: 'custom_message', ...message(reason) }], continue: true };
      return undefined;
    });
  }
}
