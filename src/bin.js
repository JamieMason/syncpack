#!/usr/bin/env node

import 'nodent-runtime';
import program from 'commander';
import { version } from '../package.json';
import * as log from './lib/log';
import syncpack from './index';

let patternValue;

program.version(version).arguments('[pattern]').action(pattern => {
  patternValue = pattern;
});

program.parse(process.argv);

syncpack({
  pattern: patternValue
}).catch(err => {
  log.bug('uncaught error in syncpack', err);
  process.exit(1);
});
