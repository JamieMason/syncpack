#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { relative } from 'path';
import { DEFAULT_PATTERN, OPTION_PACKAGES } from './constants';
import { setVersionsToNewestMismatch } from './manifests';

program.option(OPTION_PACKAGES.spec, OPTION_PACKAGES.description).parse(process.argv);

const { packages = DEFAULT_PATTERN } = program;

setVersionsToNewestMismatch(packages).then((descriptors) => {
  _.each(descriptors, (descriptor) => {
    console.log(chalk.blue(`./${relative('.', descriptor.path)}`));
  });
});
