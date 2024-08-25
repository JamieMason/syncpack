#!/usr/bin/env node

import chalk from 'chalk-template';
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { fixMismatches } from './fix-mismatches.js';

program.description(
  chalk`
  Ensure that multiple packages requiring the same dependency define the same
  version, so that every package requires eg. {yellow react@16.4.2}, instead of a
  combination of {yellow react@16.4.2}, {yellow react@0.15.9}, and {yellow react@16.0.0}.`.replace(
    /^\n/,
    '',
  ),
);

program.on('--help', () => {});

showHelpOnError(program);

program
  .option(...option.source)
  .option(...option.filter)
  .option(...option.specs)
  .option(...option.types)
  .option(...option.config)
  .option(...option.indent)
  .parse(process.argv);

Effect.runPromise(
  fixMismatches({
    io,
    cli: {
      configPath: program.opts().config,
      filter: program.opts().filter,
      indent: program.opts().indent,
      source: program.opts().source,
      specs: program.opts().specs,
      types: program.opts().types,
    },
  }),
);
