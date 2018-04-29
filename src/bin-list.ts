#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { LIST } from './constants';
import { getVersions } from './manifests';

let packages: string[] = [];

program
  .action((...args) => {
    packages = args.filter((arg) => arg && typeof arg === 'string');
  })
  .parse(process.argv);

const patterns: string[] = packages.length ? packages : LIST.defaultPatterns;

getVersions(...patterns).then((versionByName) => {
  _.each(versionByName, (versions, name) => {
    if (versions.length > 1) {
      console.log(chalk.yellow(name), chalk.dim(versions.join(', ')));
    } else {
      console.log(chalk.blue(name), chalk.dim(versions[0]));
    }
  });
});
