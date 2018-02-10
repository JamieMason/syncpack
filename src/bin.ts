#!/usr/bin/env node

import * as program from 'commander';
import { FIX_MISMATCHES, FORMAT, LIST, LIST_MISMATCHES, VERSION } from './constants';

program
  .version(VERSION)
  .command(FIX_MISMATCHES.name, FIX_MISMATCHES.description)
  .command(FORMAT.name, FORMAT.description)
  .command(LIST.name, LIST.description, { isDefault: true })
  .command(LIST_MISMATCHES.name, LIST_MISMATCHES.description);

program.parse(process.argv);
