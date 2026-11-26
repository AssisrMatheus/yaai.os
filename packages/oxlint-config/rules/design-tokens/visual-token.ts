// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';
const message = messageFor(
  'oxlint::yaai/visual-token',
  'A component style uses a literal visual value. Use a named design token.'
);
export const visualToken = {
  create(context) {
    return {
      Property(node) {
        if (
          !/\.[jt]sx$/.test(context.filename) ||
          node.key.type !== 'Identifier' ||
          ![
            'color',
            'backgroundColor',
            'borderColor',
            'borderRadius',
            'boxShadow',
            'transitionDuration',
            'transitionTimingFunction',
            'animationDuration',
            'animationTimingFunction'
          ].includes(node.key.name)
        )
          return;
        const container = node.parent.parent;
        if (container?.type !== 'JSXExpressionContainer') return;
        const attribute = container.parent;
        if (
          attribute.type !== 'JSXAttribute' ||
          attribute.name.type !== 'JSXIdentifier' ||
          attribute.name.name !== 'style'
        )
          return;
        if (node.value.type !== 'Literal') return;
        const value = node.value.value;
        if (
          typeof value === 'string' &&
          /^(?:var\(|inherit$|initial$|unset$|none$|transparent$|currentColor$)/i.test(value)
        )
          return;
        if ((typeof value === 'number' && value !== 0) || (typeof value === 'string' && value.trim()))
          context.report({ node, message });
      }
    };
  }
} satisfies Rule;
