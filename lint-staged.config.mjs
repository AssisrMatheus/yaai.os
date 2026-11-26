// What runs on staged files before a commit. spec: docs/repo/commits.md
const config = {
  '*.{js,cjs,mjs,ts,cts,mts,jsx,tsx}': ['oxlint --deny-warnings --fix', 'oxfmt --no-error-on-unmatched-pattern'],
  '*.{json,jsonc,yaml,yml,toml,md}': ['oxfmt --no-error-on-unmatched-pattern'],
  '*.rs': ['rustfmt --edition 2024']
};

export default config;
