#!/usr/bin/env node

import chalk from 'chalk-template';
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { lintSemverRanges } from './lint-semver-ranges.js';

program.description(
  chalk`
  Check dependency versions within {yellow dependencies}, {yellow devDependencies},
  {yellow peerDependencies}, {yellow overrides}, and {yellow resolutions} follow a consistent format.`.replace(
    /^\n/,
    '',
  ),
);

program.on('--help', () => {});

showHelpOnError(program);

program
  .option(...option.source)
  .option(...option.filter)
  .option(...option.config)
  .option(...option.specs)
  .option(...option.types)
  .parse(process.argv);

Effect.runPromise(
  lintSemverRanges({
    io,
    cli: {
      configPath: program.opts().config,
      filter: program.opts().filter,
      source: program.opts().source,
      specs: program.opts().specs,
      types: program.opts().types,
    },
  }),
);
