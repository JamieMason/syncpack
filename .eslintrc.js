module.exports = {
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint', 'import'],
  extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended'],
  parserOptions: {
    ecmaVersion: 6,
    sourceType: 'module',
  },
  env: {
    es6: true,
    node: true,
  },
  overrides: [
    {
      files: ['*.spec.ts'],
      env: {
        jest: true,
      },
    },
  ],
  rules: {
    '@typescript-eslint/no-namespace': 0,
    '@typescript-eslint/no-var-requires': 0,
    '@typescript-eslint/consistent-type-imports': [
      2,
      { prefer: 'type-imports' },
    ],
  },
};
