#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import _ = require('lodash');
import { relative } from 'path';
import { OPTION_SOURCES } from './constants';
import { setVersionsToNewestMismatch } from './manifests';

const collect = (value: string, values: string[] = []) => values.concat(value);

program
  .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
  .parse(process.argv);

const sources: string[] =
  program.source && program.source.length
    ? program.source
    : OPTION_SOURCES.default;

setVersionsToNewestMismatch(...sources).then((descriptors) => {
  _.each(descriptors, (descriptor) => {
    console.log(chalk.blue(`./${relative('.', descriptor.path)}`));
  });
});
