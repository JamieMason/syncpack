#!/usr/bin/env node

import * as program from 'commander';
import { COMMAND_LIST, COMMAND_LIST_MISMATCHES } from './constants';

program
  .version('TODO')
  .command(COMMAND_LIST.name, COMMAND_LIST.description, {
    isDefault: true
  })
  .command(COMMAND_LIST_MISMATCHES.name, COMMAND_LIST_MISMATCHES.description);

program.parse(process.argv);
