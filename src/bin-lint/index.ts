#!/usr/bin/env node
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { lint } from './lint.js';

program.description('  lint all versions and ranges');

program.on('--help', () => {});

showHelpOnError(program);

program.option(...option.config).parse(process.argv);

Effect.runPromise(
  lint({
    io,
    cli: {
      configPath: program.opts().config,
    },
  }),
);
