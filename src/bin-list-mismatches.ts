#!/usr/bin/env node

import chalk from 'chalk';
import * as program from 'commander';
import * as _ from 'lodash';
import { DEFAULT_PATTERN, OPTION_PACKAGES } from './constants';
import { getMismatchedVersions } from './manifests';

program.option(OPTION_PACKAGES.spec, OPTION_PACKAGES.description).parse(process.argv);

const { packages = DEFAULT_PATTERN } = program;

getMismatchedVersions(packages).then((versionByName) => {
  _.each(versionByName, (versions, name) => {
    console.log(chalk.yellow(name), chalk.dim(versions.join(', ')));
  });
});
