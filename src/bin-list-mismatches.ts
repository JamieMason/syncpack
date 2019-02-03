#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import { run } from './list-mismatches';

program.description(
  `
  List dependencies which are required by multiple packages, where the version
  is not the same across every package.`.replace(/^\n/, '')
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses packages defined in lerna.json by default')}
  syncpack list-mismatches
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack list-mismatches --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack list-mismatches --source ${chalk.yellow(
    '"apps/*/package.json"'
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# only list "devDependencies"')}
  syncpack list-mismatches --dev
  ${chalk.grey('# only list "devDependencies" and "peerDependencies"')}
  syncpack list-mismatches --dev --peer
  `);
  console.log(`Reference:
  lerna.json
  ${chalk.blue.underline('https://github.com/lerna/lerna#lernajson')}
  globs
  ${chalk.blue.underline('https://github.com/isaacs/node-glob#glob-primer')}`);
});

run(program);
