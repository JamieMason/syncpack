#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import { run } from './format';

program.description(
  `
  Organise package.json files according to a conventional format, where fields
  appear in a predictable order and nested fields are ordered alphabetically.
  Shorthand properties are used where available, such as the "repository" and
  "bugs" fields.`.replace(/^\n/, ''),
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses defaults for resolving packages')}
  syncpack format
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack format --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack format --source ${chalk.yellow(
    '"apps/*/package.json"',
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# indent package.json with 4 spaces instead of 2')}
  syncpack format --indent ${chalk.yellow('"    "')}
  `);
  console.log(`Resolving Packages:
  1. If ${chalk.yellow(`--source`)} globs are provided, use those.
  2. If using Yarn Workspaces, read ${chalk.yellow(
    `workspaces`,
  )} from ${chalk.yellow(`package.json`)}.
  3. If using Lerna, read ${chalk.yellow(`packages`)} from ${chalk.yellow(
    `lerna.json`,
  )}.
  4. Default to ${chalk.yellow(`"package.json"`)} and ${chalk.yellow(
    `"packages/*/package.json"`,
  )}.
  `);
  console.log(`Reference:
  globs            ${chalk.blue.underline(
    'https://github.com/isaacs/node-glob#glob-primer',
  )}
  lerna.json       ${chalk.blue.underline(
    'https://github.com/lerna/lerna#lernajson',
  )}
  Yarn Workspaces  ${chalk.blue.underline(
    'https://yarnpkg.com/lang/en/docs/workspaces',
  )}`);
});

run(program);
