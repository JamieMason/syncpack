#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import _ = require('lodash');
import { OPTION_SOURCES } from './constants';
import { getVersions } from './manifests';

const collect = (value: string, values: string[] = []) => values.concat(value);

program
  .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
  .parse(process.argv);

const sources: string[] =
  program.source && program.source.length
    ? program.source
    : OPTION_SOURCES.default;

getVersions(...sources).then((versionByName) => {
  _.each(versionByName, (versions, name) => {
    if (versions.length > 1) {
      console.log(chalk.yellow(name), chalk.dim(versions.join(', ')));
    } else {
      console.log(chalk.blue(name), chalk.dim(versions[0]));
    }
  });
});
