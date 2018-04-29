#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { relative } from 'path';
import { OPTION_SEMVER_RANGE, SET_SEMVER_RANGES } from './constants';
import { setVersionRange } from './manifests';

let packages: string[] = [];

program
  .command(`${SET_SEMVER_RANGES.command} ${SET_SEMVER_RANGES.args}`)
  .option(OPTION_SEMVER_RANGE.spec, OPTION_SEMVER_RANGE.description)
  .action((...args) => {
    packages = args.filter((arg) => arg && typeof arg === 'string');
  })
  .parse(process.argv);

const semverRange: string = program.semverRange || OPTION_SEMVER_RANGE.default;
const patterns: string[] = packages.length ? packages : SET_SEMVER_RANGES.defaultPatterns;

setVersionRange(semverRange, ...patterns).then((descriptors) => {
  _.each(descriptors, (descriptor) => {
    console.log(chalk.blue(`./${relative('.', descriptor.path)}`));
  });
});
