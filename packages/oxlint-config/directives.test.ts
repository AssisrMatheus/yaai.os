// spec: docs/repo/lint-rules.md#diagnostics
import { expect, test } from 'bun:test';
import { directiveLines } from './directives.ts';
test('whole-file disable cannot suppress its own policy', () => {
  expect(directiveLines('fixture.ts', '/* oxlint-disable */\nconst n = 1;')).toEqual([1]);
  expect(directiveLines('fixture.ts', 'const s = "/* oxlint-disable */";')).toEqual([]);
  expect(directiveLines('fixture.ts', '// oxlint-disable-next-line no-console -- CLI output\nconsole.log(1)')).toEqual(
    []
  );
});
