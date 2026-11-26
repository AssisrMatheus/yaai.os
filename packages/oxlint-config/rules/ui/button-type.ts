// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';
const message = messageFor(
  'oxlint::yaai/button-type',
  'A native button has an implicit submit action. Set its type explicitly.'
);
export const buttonType = {
  create(context) {
    return {
      JSXOpeningElement(node) {
        if (
          node.name.type === 'JSXIdentifier' &&
          node.name.name === 'button' &&
          !node.attributes.some(
            (a) => a.type === 'JSXSpreadAttribute' || (a.name.type === 'JSXIdentifier' && a.name.name === 'type')
          )
        )
          context.report({ node, message });
      }
    };
  }
} satisfies Rule;
