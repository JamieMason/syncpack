#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { LIST_MISMATCHES } from './constants';
import { getMismatchedVersions } from './manifests';

let packages: string[] = [];

program
  .action((...args) => {
    packages = args.filter((arg) => arg && typeof arg === 'string');
  })
  .parse(process.argv);

const patterns: string[] = packages.length ? packages : LIST_MISMATCHES.defaultPatterns;

getMismatchedVersions(...patterns).then((versionByName) => {
  _.each(versionByName, (versions, name) => {
    console.log(chalk.yellow(name), chalk.dim(versions.join(', ')));
  });

  if (versionByName.length) {
    process.exit(1);
  }
});
