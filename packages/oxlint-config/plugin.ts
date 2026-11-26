// spec: docs/repo/lint-rules.md#bindings
import { rules as jsdocRules } from 'eslint-plugin-jsdoc';
import { booleanState } from './rules/typescript/boolean-state.ts';
import { noEnum } from './rules/typescript/no-enum.ts';
import { suppressionReason } from './rules/typescript/suppression-reason.ts';
import { transitionProperties } from './rules/motion/transition-properties.ts';
import { translatedCopy } from './rules/i18n/translated-copy.ts';
import { visualToken } from './rules/design-tokens/visual-token.ts';
import { verifyTls } from './rules/security/verify-tls.ts';
import { buttonType } from './rules/ui/button-type.ts';
import { workspaceImports } from './rules/code-organization/workspace-imports.ts';
import { environmentRule } from './rules/configuration/environment.ts';
const descriptionRule = jsdocRules?.['require-description'];
const documentationRule = jsdocRules?.['require-jsdoc'];
if (!descriptionRule || !documentationRule) throw new Error('Install the workspace version of eslint-plugin-jsdoc.');
// oxlint-disable-next-line import/no-default-export -- Oxlint loads plugins through the module default export.
export default {
  meta: { name: 'yaai' },
  rules: {
    'boolean-state': booleanState,
    'require-description': descriptionRule,
    'require-jsdoc': documentationRule,
    'env-template': environmentRule('template'),
    'task-env': environmentRule('task'),
    'workspace-imports': workspaceImports,
    'no-enum': noEnum,
    'suppression-reason': suppressionReason,
    'transition-properties': transitionProperties,
    'translated-copy': translatedCopy,
    'visual-token': visualToken,
    'verify-tls': verifyTls,
    'button-type': buttonType
  }
};
