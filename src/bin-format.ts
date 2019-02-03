#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import { run } from './format';

program.description(
  `
  Organise package.json files according to a conventional format, where fields
  appear in a predictable order and nested fields are ordered alphabetically.
  Shorthand properties are used where available, such as the "repository" and
  "bugs" fields.`.replace(/^\n/, '')
);

program.on('--help', () => {
  console.log('');
  console.log(`Examples:
  ${chalk.grey('# uses packages defined in lerna.json by default')}
  syncpack format
  ${chalk.grey('# uses packages defined by --source when provided')}
  syncpack format --source ${chalk.yellow('"apps/*/package.json"')}
  ${chalk.grey('# multiple globs can be provided like this')}
  syncpack format --source ${chalk.yellow(
    '"apps/*/package.json"'
  )} --source ${chalk.yellow('"core/*/package.json"')}
  ${chalk.grey('# indent package.json with 4 spaces instead of 2')}
  syncpack format --indent ${chalk.yellow('"    "')}
  `);
  console.log(`Reference:
  lerna.json
  ${chalk.blue.underline('https://github.com/lerna/lerna#lernajson')}
  globs
  ${chalk.blue.underline('https://github.com/isaacs/node-glob#glob-primer')}`);
});

run(program);
