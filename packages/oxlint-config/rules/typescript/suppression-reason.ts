// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';

export const suppressionMessage = messageFor(
  'oxlint::yaai/suppression-reason',
  'A suppression lacks a one-line scope, lint name or reason. Use oxlint-disable-next-line with a rule and -- reason.'
);

/** Identify suppressions missing a rule, reason or one-line scope. */
export function invalidDirective(comment: string) {
  const directive = comment.trim();
  return (
    /^(?:oxlint|eslint)-disable\b/.test(directive) &&
    !/^oxlint-disable-next-line\s+[\w/@-]+(?:\s*,\s*[\w/@-]+)*\s+--\s+\S/.test(directive)
  );
}

export const suppressionReason = {
  create(context) {
    return {
      Program() {
        for (const comment of context.sourceCode.getAllComments()) {
          if (invalidDirective(comment.value)) context.report({ node: comment, message: suppressionMessage });
        }
      }
    };
  }
} satisfies Rule;
