#!/usr/bin/env node
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { update } from './update.js';

program.description('  Update to the latest versions on the npm registry.');

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
  update(io, {
    configPath: program.opts().config,
    filter: program.opts().filter,
    source: program.opts().source,
    specs: program.opts().specs,
    types: program.opts().types,
  }),
);
