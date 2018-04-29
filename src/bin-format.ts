#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { relative } from 'path';
import { FORMAT } from './constants';
import { format } from './manifests';

let packages: string[] = [];

program
  .command(`${FORMAT.command} ${FORMAT.args}`)
  .action((...args) => {
    packages = args.filter((arg) => arg && typeof arg === 'string');
  })
  .parse(process.argv);

const patterns: string[] = packages.length ? packages : FORMAT.defaultPatterns;

format(...patterns).then((descriptors) => {
  _.each(descriptors, (descriptor) => {
    console.log(chalk.blue(`./${relative('.', descriptor.path)}`));
  });
});
