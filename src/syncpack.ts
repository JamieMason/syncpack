import { CommanderStatic } from 'commander';
import {
  FIX_MISMATCHES,
  FORMAT,
  LIST,
  LIST_MISMATCHES,
  SET_SEMVER_RANGES,
  VERSION
} from './constants';

export const run = (program: CommanderStatic) => {
  program
    .version(VERSION)
    .command(FIX_MISMATCHES.command, FIX_MISMATCHES.description)
    .command(FORMAT.command, FORMAT.description)
    .command(LIST.command, LIST.description, { isDefault: true })
    .command(LIST_MISMATCHES.command, LIST_MISMATCHES.description)
    .command(SET_SEMVER_RANGES.command, SET_SEMVER_RANGES.description);

  program.parse(process.argv);
};
