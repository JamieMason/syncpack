#!/usr/bin/env node

import * as program from 'commander';
import { COMMAND_LIST } from './constants';

program.version('TODO').command(COMMAND_LIST.name, COMMAND_LIST.description, {
  isDefault: true
});

program.parse(process.argv);
