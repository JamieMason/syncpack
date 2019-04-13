import chalk from 'chalk';
import { writeJson } from 'fs-extra';
import * as _ from 'lodash';
import { relative } from 'path';
import {
  OPTION_INDENT,
  OPTION_SOURCES,
  OPTIONS_DEV,
  OPTIONS_FILTER_DEPENDENCIES,
  OPTIONS_PEER,
  OPTIONS_PROD
} from './constants';
import { collect } from './lib/collect';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getIndent } from './lib/get-indent';
import { getPackages } from './lib/get-packages';
import { getMismatchedVersionsByName } from './lib/get-versions-by-name';
import { getNewest } from './lib/version';
import { CommanderApi, IManifest } from './typings';

export const run = async (program: CommanderApi) => {
  program
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .option(OPTIONS_PROD.spec, OPTIONS_PROD.description)
    .option(OPTIONS_DEV.spec, OPTIONS_DEV.description)
    .option(OPTIONS_PEER.spec, OPTIONS_PEER.description)
    .option(OPTION_INDENT.spec, OPTION_INDENT.description)
    .option(
      OPTIONS_FILTER_DEPENDENCIES.spec,
      OPTIONS_FILTER_DEPENDENCIES.description
    )
    .parse(process.argv);

  const pkgs = getPackages(program);
  const dependencyTypes = getDependencyTypes(program);
  const indent = getIndent(program);
  const mismatchedVersionsByName = getMismatchedVersionsByName(
    dependencyTypes,
    pkgs,
    program.filter
  );

  await Promise.all(
    pkgs.map(({ data, path }) => {
      const nextData: IManifest = JSON.parse(JSON.stringify(data));
      const shortPath = `./${relative('.', path)}`;
      const changes: string[][] = [];
      Object.entries(mismatchedVersionsByName).forEach(([name, versions]) => {
        const newest = getNewest(versions);
        dependencyTypes.forEach((type) => {
          if (
            nextData[type] &&
            nextData[type][name] &&
            nextData[type][name] !== newest
          ) {
            changes.push([name, nextData[type][name], newest]);
            nextData[type][name] = newest;
          }
        });
      });
      if (changes.length > 0) {
        changes.forEach(([name, from, to]) => {
          console.log(
            chalk.bgYellow.black(' FIXED '),
            chalk.blue(shortPath),
            name,
            chalk.red(from),
            '→',
            chalk.green(to)
          );
        });
        return writeJson(path, nextData, { spaces: indent });
      }
      console.log(chalk.bgGreen.black(' VALID '), chalk.blue(shortPath));
    })
  );
};
