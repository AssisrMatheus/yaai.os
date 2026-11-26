// spec: docs/repo/lint-rules.md#diagnostics
import { type Rule, messageFor } from '../../bindings.ts';
const message = messageFor(
  'oxlint::yaai/verify-tls',
  'TLS verification is disabled. Configure a trusted certificate authority.'
);
export const verifyTls = {
  create(context) {
    return {
      AssignmentExpression(node) {
        if (
          node.left.type !== 'MemberExpression' ||
          node.right.type !== 'Literal' ||
          (node.right.value !== '0' && node.right.value !== 0)
        )
          return;
        const target = context.sourceCode.getText(node.left);
        if (
          /^(?:process|Bun)\.env(?:\.NODE_TLS_REJECT_UNAUTHORIZED|\[['"]NODE_TLS_REJECT_UNAUTHORIZED['"]\])$/.test(
            target
          )
        )
          context.report({ node, message });
      },
      Property(node) {
        if (
          node.key.type === 'Identifier' &&
          node.key.name === 'rejectUnauthorized' &&
          node.value.type === 'Literal' &&
          node.value.value === false
        )
          context.report({ node, message });
      }
    };
  }
} satisfies Rule;
