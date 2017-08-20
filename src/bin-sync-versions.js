#!/usr/bin/env node

import 'nodent-runtime';
import program from 'commander';
import * as log from './lib/log';
import syncVersions from './sync-versions';
import { DEFAULT_PACKAGES } from './constants';

program
  .option('-p, --packages <glob>', `location of packages. defaults to ${DEFAULT_PACKAGES}`)
  .parse(process.argv);

const { packages = DEFAULT_PACKAGES } = program;

syncVersions({
  packagesPattern: packages
}).catch(err => {
  log.bug('uncaught error in syncVersions', err);
  process.exit(1);
});
