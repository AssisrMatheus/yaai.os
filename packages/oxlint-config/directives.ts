// spec: docs/repo/lint-rules.md#diagnostics
import { readFileSync } from 'node:fs';
import { parseSync } from 'oxc-parser';
import { invalidDirective, suppressionMessage } from './rules/typescript/suppression-reason.ts';

// A blanket disable silences the plugin that validates it. Inspect parsed comments independently.
/** Locate invalid suppression comments, including file-wide disables. */
export function directiveLines(path: string, source: string) {
  return parseSync(path, source)
    .comments.filter((comment) => invalidDirective(comment.value))
    .map((comment) => source.slice(0, comment.start).split(/\r\n|[\r\n\u2028\u2029]/u).length);
}

if (import.meta.main) {
  const rows = process.argv
    .slice(2)
    .flatMap((path) =>
      directiveLines(path, readFileSync(path, 'utf8')).map((line) => ({ path, line, message: suppressionMessage }))
    );
  process.stdout.write(JSON.stringify(rows));
}
