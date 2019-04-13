#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import { run } from './fix-mismatches';

program.description(
  `
  Ensure that multiple packages requiring the same dependency define the same
  version, so that every package requires eg. react@16.4.2, instead of a
  combination of react@16.4.2, react@0.15.9, and react@16.0.0.`.replace(
    /^\n/,
    ''
  )
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses packages defined in lerna.json by default')}
  syncpack fix-mismatches
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack fix-mismatches --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey(
    '# uses dependencies regular expression defined by --filter when provided'
  )}
  syncpack fix-mismatches --filter ${chalk.yellow('"typescript|tslint"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack fix-mismatches --source ${chalk.yellow(
    '"apps/*/package.json"'
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# only fix "devDependencies"')}
  syncpack fix-mismatches --dev
  ${chalk.grey('# only fix "devDependencies" and "peerDependencies"')}
  syncpack fix-mismatches --dev --peer
  ${chalk.grey('# indent package.json with 4 spaces instead of 2')}
  syncpack fix-mismatches --indent ${chalk.yellow('"    "')}
  `);
  console.log(`Reference:
  lerna.json
  ${chalk.blue.underline('https://github.com/lerna/lerna#lernajson')}
  globs
  ${chalk.blue.underline('https://github.com/isaacs/node-glob#glob-primer')}`);
});

run(program);
