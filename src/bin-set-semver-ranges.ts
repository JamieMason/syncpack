#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import { run } from './set-semver-ranges';

program.description(
  `
  Ensure dependency versions used within "dependencies", "devDependencies", and
  "peerDependencies" follow a consistent format.`.replace(/^\n/, '')
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses packages defined in lerna.json by default')}
  syncpack set-semver-ranges
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack set-semver-ranges --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack set-semver-ranges --source ${chalk.yellow(
    '"apps/*/package.json"'
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
  console.log(`Reference:
  lerna.json
  ${chalk.blue.underline('https://github.com/lerna/lerna#lernajson')}
  globs
  ${chalk.blue.underline('https://github.com/isaacs/node-glob#glob-primer')}`);
});

run(program);
