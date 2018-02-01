#!/usr/bin/env node

import * as program from 'commander';
import { FIX_MISMATCHES, LIST, LIST_MISMATCHES } from './constants';

program
  .version('TODO')
  .command(FIX_MISMATCHES.name, FIX_MISMATCHES.description)
  .command(LIST.name, LIST.description, { isDefault: true })
  .command(LIST_MISMATCHES.name, LIST_MISMATCHES.description);

program.parse(process.argv);
