#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import { run } from './fix-mismatches';

program.description(
  `
  Ensure that multiple packages requiring the same dependency define the same
  version, so that every package requires eg. react@16.4.2, instead of a
  combination of react@16.4.2, react@0.15.9, and react@16.0.0.`.replace(
    /^\n/,
    '',
  ),
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses defaults for resolving packages')}
  syncpack fix-mismatches
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack fix-mismatches --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey(
    '# uses dependencies regular expression defined by --filter when provided',
  )}
  syncpack fix-mismatches --filter ${chalk.yellow('"typescript|tslint"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack fix-mismatches --source ${chalk.yellow(
    '"apps/*/package.json"',
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# only fix "devDependencies"')}
  syncpack fix-mismatches --dev
  ${chalk.grey('# only fix "devDependencies" and "peerDependencies"')}
  syncpack fix-mismatches --dev --peer
  ${chalk.grey('# indent package.json with 4 spaces instead of 2')}
  syncpack fix-mismatches --indent ${chalk.yellow('"    "')}
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
