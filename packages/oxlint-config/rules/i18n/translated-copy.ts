// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';
const message = messageFor(
  'oxlint::yaai/translated-copy',
  'Literal interface copy bypasses translations. Render the appropriate message key.'
);
export const translatedCopy = {
  create(context) {
    return {
      JSXText(node) {
        if (/[\p{L}\p{N}]/u.test(node.value)) context.report({ node, message });
      },
      JSXAttribute(node) {
        if (
          node.name.type === 'JSXIdentifier' &&
          ['placeholder', 'aria-label', 'title'].includes(node.name.name) &&
          node.value?.type === 'Literal' &&
          typeof node.value.value === 'string' &&
          node.value.value.trim()
        )
          context.report({ node, message });
      }
    };
  }
} satisfies Rule;
