#!/usr/bin/env node

import * as program from 'commander';
import { FIX_MISMATCHES, FORMAT, LIST, LIST_MISMATCHES, SET_SEMVER_RANGES, VERSION } from './constants';

program
  .version(VERSION)
  .command(FIX_MISMATCHES.name, FIX_MISMATCHES.description)
  .command(FORMAT.name, FORMAT.description)
  .command(LIST.name, LIST.description, { isDefault: true })
  .command(LIST_MISMATCHES.name, LIST_MISMATCHES.description)
  .command(SET_SEMVER_RANGES.name, SET_SEMVER_RANGES.description);

program.parse(process.argv);
