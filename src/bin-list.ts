#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import { run } from './list';

program.description(
  `
  List all dependencies required by your packages.`.replace(/^\n/, '')
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses defaults for resolving packages')}
  syncpack list
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack list --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey(
    '# uses dependencies regular expression defined by --filter when provided'
  )}
  syncpack list --filter ${chalk.yellow('"typescript|tslint"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack list --source ${chalk.yellow(
    '"apps/*/package.json"'
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# only inspect "devDependencies"')}
  syncpack list --dev
  ${chalk.grey('# only inspect "devDependencies" and "peerDependencies"')}
  syncpack list --dev --peer
  `);
  console.log(`Resolving Packages:
  1. If ${chalk.yellow(`--source`)} globs are provided, use those.
  2. If using Yarn Workspaces, read ${chalk.yellow(
    `workspaces`
  )} from ${chalk.yellow(`package.json`)}.
  3. If using Lerna, read ${chalk.yellow(`packages`)} from ${chalk.yellow(
    `lerna.json`
  )}.
  4. Default to ${chalk.yellow(`"package.json"`)} and ${chalk.yellow(
    `"packages/*/package.json"`
  )}.
  `);
  console.log(`Reference:
  globs            ${chalk.blue.underline(
    'https://github.com/isaacs/node-glob#glob-primer'
  )}
  lerna.json       ${chalk.blue.underline(
    'https://github.com/lerna/lerna#lernajson'
  )}
  Yarn Workspaces  ${chalk.blue.underline(
    'https://yarnpkg.com/lang/en/docs/workspaces'
  )}`);
});

run(program);
