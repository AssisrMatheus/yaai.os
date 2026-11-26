// spec: docs/repo/lint-rules.md#bindings
import type { RuleTester } from 'oxlint/plugins-dev';
import catalog from '@yaai/lints/bindings' with { type: 'json' };

export type Rule = Parameters<RuleTester['run']>[1];
/** Append the canonical best-practice references to a diagnostic. */
export function messageFor(key: keyof typeof catalog, action: string) {
  return action + ' Rule: ' + catalog[key].join(', ');
}
