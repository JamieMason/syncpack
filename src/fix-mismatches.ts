import chalk from 'chalk';
import { CommanderStatic } from 'commander';
import _ = require('lodash');
import { relative } from 'path';
import {
  OPTION_SOURCES,
  OPTIONS_DEV,
  OPTIONS_PEER,
  OPTIONS_PROD
} from './constants';
import { collect } from './lib/collect';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getPackages } from './lib/get-packages';
import { getMismatchedVersionsByName } from './lib/get-versions-by-name';
import { getNewest } from './lib/version';
import { writeJson } from './lib/write-json';

export const run = async (program: CommanderStatic) => {
  program
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .option(OPTIONS_PROD.spec, OPTIONS_PROD.description)
    .option(OPTIONS_DEV.spec, OPTIONS_DEV.description)
    .option(OPTIONS_PEER.spec, OPTIONS_PEER.description)
    .parse(process.argv);

  const dependencyTypes = getDependencyTypes(program);
  const pkgs = await getPackages(program);
  const mismatchedVersionsByName = getMismatchedVersionsByName(
    dependencyTypes,
    pkgs
  );

  Object.entries(mismatchedVersionsByName).forEach(([name, versions]) => {
    const newest = getNewest(versions);
    if (typeof newest === 'string') {
      pkgs.forEach(({ data, path }) => {
        dependencyTypes.forEach((type) => {
          if (data[type] && data[type][name] && data[type][name] !== newest) {
            console.log(
              relative(process.cwd(), path),
              name,
              data[type][name],
              '->',
              newest
            );
            data[type][name] = newest;
          }
        });
      });
    }
  });

  await Promise.all(pkgs.map(({ data, path }) => writeJson(path, data)));

  _.each(pkgs, (pkg) => {
    console.log(chalk.blue(`./${relative('.', pkg.path)}`));
  });
};
