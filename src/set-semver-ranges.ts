import chalk from 'chalk';
import { CommanderStatic } from 'commander';
import _ = require('lodash');
import { relative } from 'path';
import semver = require('semver');
import {
  OPTION_SEMVER_RANGE,
  OPTION_SOURCES,
  OPTIONS_DEV,
  OPTIONS_PEER,
  OPTIONS_PROD,
  RANGE_ANY,
  RANGE_LOOSE
} from './constants';
import { collect } from './lib/collect';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getPackages } from './lib/get-packages';
import { getVersionNumber } from './lib/version';
import { writeJson } from './lib/write-json';

export const run = async (program: CommanderStatic) => {
  program
    .option(OPTION_SEMVER_RANGE.spec, OPTION_SEMVER_RANGE.description)
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .option(OPTIONS_PROD.spec, OPTIONS_PROD.description)
    .option(OPTIONS_DEV.spec, OPTIONS_DEV.description)
    .option(OPTIONS_PEER.spec, OPTIONS_PEER.description)
    .parse(process.argv);

  const semverRange: string =
    program.semverRange || OPTION_SEMVER_RANGE.default;

  const dependencyTypes = getDependencyTypes(program);
  const pkgs = await getPackages(program);

  _(pkgs).each((pkg) =>
    _(dependencyTypes)
      .map((property) => pkg.data[property])
      .filter(Boolean)
      .each((dependencies) => {
        _(dependencies).each((version, name) => {
          const versionNumber = getVersionNumber(version);
          if (version !== '*' && semver.validRange(version)) {
            if (semverRange === RANGE_ANY) {
              dependencies[name] = '*';
            } else if (semverRange === RANGE_LOOSE) {
              dependencies[name] =
                semver.major(versionNumber) === 0
                  ? `${semver.major(versionNumber)}.${semver.minor(
                      versionNumber
                    )}.x`
                  : `${semver.major(versionNumber)}.x.x`;
            } else {
              dependencies[name] = `${semverRange}${versionNumber}`;
            }
          }
        });
      })
  );

  await Promise.all(pkgs.map(({ data, path }) => writeJson(path, data)));

  _.each(pkgs, (pkg) => {
    console.log(chalk.blue(`./${relative('.', pkg.path)}`));
  });
};
