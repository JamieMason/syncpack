#!/usr/bin/env node
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { prompt } from './prompt.js';

program.description(
  '  displays a series of prompts to fix mismatches which syncpack cannot fix automatically',
);

program.on('--help', () => {});

showHelpOnError(program);

program
  .option(...option.source)
  .option(...option.filter)
  .option(...option.config)
  .option(...option.specs)
  .option(...option.types)
  .option(...option.indent)
  .parse(process.argv);

Effect.runPromise(
  prompt({
    io,
    cli: {
      configPath: program.opts().config,
      filter: program.opts().filter,
      source: program.opts().source,
      specs: program.opts().specs,
      types: program.opts().types,
      indent: program.opts().indent,
    },
  }),
);
