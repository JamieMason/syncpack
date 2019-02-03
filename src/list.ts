import chalk from 'chalk';
import * as _ from 'lodash';
import {
  OPTION_SOURCES,
  OPTIONS_DEV,
  OPTIONS_PEER,
  OPTIONS_PROD
} from './constants';
import { collect } from './lib/collect';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getPackages } from './lib/get-packages';
import { getVersionsByName } from './lib/get-versions-by-name';
import { CommanderApi } from './typings';

export const run = async (program: CommanderApi) => {
  program
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .option(OPTIONS_PROD.spec, OPTIONS_PROD.description)
    .option(OPTIONS_DEV.spec, OPTIONS_DEV.description)
    .option(OPTIONS_PEER.spec, OPTIONS_PEER.description)
    .parse(process.argv);

  const dependencyTypes = getDependencyTypes(program);
  const pkgs = getPackages(program);
  const versionsByName = getVersionsByName(dependencyTypes, pkgs);

  _(versionsByName)
    .entries()
    .sortBy('0')
    .each(([name, versions]) => {
      if (versions.length > 1) {
        console.log(chalk.yellow(name), chalk.dim(versions.join(', ')));
      } else {
        console.log(chalk.blue(name), chalk.dim(versions[0]));
      }
    });
};
