#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import { run } from './set-semver-ranges';

program.description(
  `
  Ensure dependency versions used within "dependencies", "devDependencies", and
  "peerDependencies" follow a consistent format.`.replace(/^\n/, ''),
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses defaults for resolving packages')}
  syncpack set-semver-ranges
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack set-semver-ranges --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack set-semver-ranges --source ${chalk.yellow(
    '"apps/*/package.json"',
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# use ~ range instead of default ""')}
  syncpack set-semver-ranges --semver-range ~
  ${chalk.grey('# set ~ range in "devDependencies"')}
  syncpack set-semver-ranges --dev --semver-range ~
  ${chalk.grey('# set ~ range in "devDependencies" and "peerDependencies"')}
  syncpack set-semver-ranges --dev --peer --semver-range ~
  ${chalk.grey('# indent package.json with 4 spaces instead of 2')}
  syncpack set-semver-ranges --indent ${chalk.yellow('"    "')}
  `);
  console.log(`Supported Ranges:
  <  ${chalk.grey('<1.4.2')}
  <= ${chalk.grey('<=1.4.2')}
  "" ${chalk.grey('1.4.2')}
  ~  ${chalk.grey('~1.4.2')}
  ^  ${chalk.grey('^1.4.2')}
  >= ${chalk.grey('>=1.4.2')}
  >  ${chalk.grey('>1.4.2')}
  *  ${chalk.grey('*')}
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
