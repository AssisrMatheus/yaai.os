// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';
export const noEnum = {
  create(context) {
    return {
      TSEnumDeclaration(node) {
        context.report({
          node,
          message: messageFor(
            'oxlint::yaai/no-enum',
            'An enum declares a closed set using a runtime-specific type. Use a literal union or an as const object.'
          )
        });
      }
    };
  }
} satisfies Rule;
