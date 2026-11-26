// spec: docs/repo/lint-rules.md#bindings
import { type Rule, messageFor } from '../../bindings.ts';

// Match Clippy's default limit for a group of boolean fields.
const maximumBooleans = 3;
export const booleanState = {
  create(context) {
    return {
      TSTypeLiteral(node) {
        const flags = node.members.filter(
          (item) =>
            item.type === 'TSPropertySignature' && item.typeAnnotation?.typeAnnotation.type === 'TSBooleanKeyword'
        );
        if (flags.length > maximumBooleans)
          context.report({
            node,
            message: messageFor(
              'oxlint::yaai/boolean-state',
              'The type groups several state flags. Model exclusive states as a discriminated union or separate independent concerns.'
            )
          });
      }
    };
  }
} satisfies Rule;
