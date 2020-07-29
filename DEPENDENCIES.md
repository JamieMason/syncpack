# syncpack

Manage multiple package.json files, such as in Lerna Monorepos and Pnpm/Yarn Workspaces

## Installation

This is a [Node.js](https://nodejs.org/) module available through the 
[npm registry](https://www.npmjs.com/). It can be installed using the 
[`npm`](https://docs.npmjs.com/getting-started/installing-npm-packages-locally),
[`pnpm`](https://pnpm.js.org/en/installation)
or 
[`yarn`](https://yarnpkg.com/en/)
command line tools.

```sh
npm install syncpack --save
```

## Tests

```sh
npm install
npm test
```

## Dependencies

- [chalk](https://ghub.io/chalk): Terminal string styling done right
- [commander](https://ghub.io/commander): the complete solution for node.js command-line programs
- [fs-extra](https://ghub.io/fs-extra): fs-extra contains methods that aren&#39;t included in the vanilla Node.js fs package. Such as recursive mkdir, copy, and remove.
- [@manypkg/get-packages](https://github.com/Thinkmill/manypkg/tree/master/packages/get-packages): a simple utility to get the packages from a monorepo, whether they're using Yarn, Bolt or pnpm
- [glob](https://ghub.io/glob): a little globber
- [semver](https://ghub.io/semver): The semantic version parser used by npm.

## Dev Dependencies

- [@types/fs-extra](https://ghub.io/@types/fs-extra): TypeScript definitions for fs-extra
- [@types/glob](https://ghub.io/@types/glob): TypeScript definitions for Glob
- [@types/jest](https://ghub.io/@types/jest): TypeScript definitions for Jest
- [@types/mock-fs](https://ghub.io/@types/mock-fs): TypeScript definitions for mock-fs
- [@types/node](https://ghub.io/@types/node): TypeScript definitions for Node.js
- [@types/semver](https://ghub.io/@types/semver): TypeScript definitions for semver
- [@typescript-eslint/eslint-plugin](https://ghub.io/@typescript-eslint/eslint-plugin): TypeScript plugin for ESLint
- [@typescript-eslint/parser](https://ghub.io/@typescript-eslint/parser): An ESLint custom parser which leverages TypeScript ESTree
- [eslint](https://ghub.io/eslint): An AST-based pattern checker for JavaScript.
- [expect-more-jest](https://ghub.io/expect-more-jest): Write Beautiful Specs with Custom Matchers for Jest
- [jest](https://ghub.io/jest): Delightful JavaScript Testing.
- [mock-fs](https://ghub.io/mock-fs): A configurable mock file system.  You know, for testing.
- [prettier](https://ghub.io/prettier): Prettier is an opinionated code formatter
- [rimraf](https://ghub.io/rimraf): A deep deletion module for node (like `rm -rf`)
- [ts-jest](https://ghub.io/ts-jest): A preprocessor with source maps support to help use TypeScript with Jest
- [typescript](https://ghub.io/typescript): TypeScript is a language for application scale JavaScript development

## License

MIT
