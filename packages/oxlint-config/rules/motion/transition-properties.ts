// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';
const message = messageFor(
  'oxlint::yaai/transition-properties',
  'A transition targets every property. Name the properties that change.'
);
export const transitionProperties = {
  create(context) {
    return {
      Property(node) {
        if (node.key.type !== 'Identifier' || !['transition', 'transitionProperty'].includes(node.key.name)) return;
        if (
          node.value.type === 'Literal' &&
          typeof node.value.value === 'string' &&
          /(?:^|[,\s])all(?:$|[,\s])/.test(node.value.value)
        )
          context.report({ node, message });
      }
    };
  }
} satisfies Rule;
