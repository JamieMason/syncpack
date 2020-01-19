import chalk from 'chalk';
import _ = require('lodash');
import {
  OPTION_SOURCES,
  OPTIONS_DEV,
  OPTIONS_FILTER_DEPENDENCIES,
  OPTIONS_PEER,
  OPTIONS_PROD,
} from './constants';
import { collect } from './lib/collect';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getPackages } from './lib/get-packages';
import { getMismatchedVersionsByName } from './lib/get-versions-by-name';
import { CommanderApi } from './typings';

export const run = async (program: CommanderApi) => {
  program
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .option(OPTIONS_PROD.spec, OPTIONS_PROD.description)
    .option(OPTIONS_DEV.spec, OPTIONS_DEV.description)
    .option(OPTIONS_PEER.spec, OPTIONS_PEER.description)
    .option(
      OPTIONS_FILTER_DEPENDENCIES.spec,
      OPTIONS_FILTER_DEPENDENCIES.description,
    )
    .parse(process.argv);
  const dependencyTypes = getDependencyTypes(program);
  const pkgs = getPackages(program);
  const mismatchedVersionsByName = getMismatchedVersionsByName(
    dependencyTypes,
    pkgs,
    program.filter,
  );

  _.each(mismatchedVersionsByName, (versions, name) => {
    console.log(chalk.yellow(name), chalk.dim(versions.join(', ')));
  });

  if (Object.keys(mismatchedVersionsByName).length) {
    process.exit(1);
  }
};
