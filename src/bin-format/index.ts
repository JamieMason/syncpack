#!/usr/bin/env node

import chalk from 'chalk-template';
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { format } from './format.js';

program.description(
  chalk`
  Organise package.json files according to a conventional format, where fields
  appear in a predictable order and nested fields are ordered alphabetically.
  Shorthand properties are used where available, such as the {yellow repository} and
  {yellow bugs} fields.`.replace(/^\n/, ''),
);

program.on('--help', () => {});

showHelpOnError(program);

program
  .option(...option.source)
  .option(...option.config)
  .option(...option.indent)
  .parse(process.argv);

Effect.runPromise(
  format({
    io,
    cli: {
      configPath: program.opts().config,
      indent: program.opts().indent,
      source: program.opts().source,
    },
  }),
);
