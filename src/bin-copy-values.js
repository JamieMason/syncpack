#!/usr/bin/env node

import 'nodent-runtime';
import program from 'commander';
import * as log from './lib/log';
import copyValues from './copy-values';
import { DEFAULT_PACKAGES, DEFAULT_SOURCE } from './constants';

program
  .option('-p, --packages <glob>', `location of packages. defaults to ${DEFAULT_PACKAGES}`)
  .option('-s, --source <glob>', `location of source. defaults to ${DEFAULT_SOURCE}`)
  .parse(process.argv);

const { args = [], packages = DEFAULT_PACKAGES, source = DEFAULT_SOURCE } = program;

copyValues({
  keys: args,
  packagesPattern: packages,
  sourcePattern: source
}).catch(err => {
  log.bug('uncaught error in copyValues', err);
  process.exit(1);
});
