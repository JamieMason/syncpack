#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { relative } from 'path';
import { DEFAULT_PATTERN, DEFAULT_SEMVER_RANGE, OPTION_PACKAGES, OPTION_SEMVER_RANGE } from './constants';
import { setVersionRange } from './manifests';

program
  .option(OPTION_PACKAGES.spec, OPTION_PACKAGES.description)
  .option(OPTION_SEMVER_RANGE.spec, OPTION_SEMVER_RANGE.description)
  .parse(process.argv);

const { packages = DEFAULT_PATTERN, semverRange = DEFAULT_SEMVER_RANGE } = program;

setVersionRange(semverRange, packages).then((descriptors) => {
  _.each(descriptors, (descriptor) => {
    console.log(chalk.blue(`./${relative('.', descriptor.path)}`));
  });
});
