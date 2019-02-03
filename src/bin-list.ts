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
  ${chalk.grey('# uses packages defined in lerna.json by default')}
  syncpack list
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack list --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack list --source ${chalk.yellow(
    '"apps/*/package.json"'
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# only inspect "devDependencies"')}
  syncpack list --dev
  ${chalk.grey('# only inspect "devDependencies" and "peerDependencies"')}
  syncpack list --dev --peer
  `);
  console.log(`Reference:
  lerna.json
  ${chalk.blue.underline('https://github.com/lerna/lerna#lernajson')}
  globs
  ${chalk.blue.underline('https://github.com/isaacs/node-glob#glob-primer')}`);
});

run(program);
