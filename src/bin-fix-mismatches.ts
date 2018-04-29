#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { relative } from 'path';
import { FIX_MISMATCHES } from './constants';
import { setVersionsToNewestMismatch } from './manifests';

let packages: string[] = [];

program
  .action((...args) => {
    packages = args.filter((arg) => arg && typeof arg === 'string');
  })
  .parse(process.argv);

const patterns: string[] = packages.length ? packages : FIX_MISMATCHES.defaultPatterns;

setVersionsToNewestMismatch(...patterns).then((descriptors) => {
  _.each(descriptors, (descriptor) => {
    console.log(chalk.blue(`./${relative('.', descriptor.path)}`));
  });
});
