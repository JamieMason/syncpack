#!/usr/bin/env node

import 'nodent-runtime';
import program from 'commander';
import { version } from '../package.json';
import * as log from './lib/log';
import syncpack from './index';

program.version(version);
program.parse(process.argv);

syncpack({}).catch(err => {
  log.bug('uncaught error in syncpack', err);
  process.exit(1);
});
