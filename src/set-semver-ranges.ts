import chalk from 'chalk';
import { writeJson } from 'fs-extra';
import * as _ from 'lodash';
import { relative } from 'path';
import * as semver from 'semver';
import {
  OPTION_INDENT,
  OPTION_SEMVER_RANGE,
  OPTION_SOURCES,
  OPTIONS_DEV,
  OPTIONS_FILTER_DEPENDENCIES,
  OPTIONS_PEER,
  OPTIONS_PROD,
  RANGE_ANY,
  RANGE_LOOSE
} from './constants';
import { collect } from './lib/collect';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getIndent } from './lib/get-indent';
import { getPackages } from './lib/get-packages';
import { getDependencyFilter } from './lib/get-versions-by-name';
import { getVersionNumber } from './lib/version';
import { CommanderApi } from './typings';

export const run = async (program: CommanderApi) => {
  program
    .option(OPTION_SEMVER_RANGE.spec, OPTION_SEMVER_RANGE.description)
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .option(OPTIONS_PROD.spec, OPTIONS_PROD.description)
    .option(OPTIONS_DEV.spec, OPTIONS_DEV.description)
    .option(OPTIONS_PEER.spec, OPTIONS_PEER.description)
    .option(
      OPTIONS_FILTER_DEPENDENCIES.spec,
      OPTIONS_FILTER_DEPENDENCIES.description
    )
    .option(OPTION_INDENT.spec, OPTION_INDENT.description)
    .parse(process.argv);

  const semverRange: string =
    program.semverRange || OPTION_SEMVER_RANGE.default;

  const pkgs = getPackages(program);
  const dependencyTypes = getDependencyTypes(program);
  const indent = getIndent(program);
  const dependencyFilter = getDependencyFilter(program.filter);
  _(pkgs).each((pkg) =>
    _(dependencyTypes)
      .map((property) => pkg.data[property])
      .filter(Boolean)
      .each((dependencies) => {
        _(dependencies).each((version, name) => {
          if (!dependencyFilter(name)) {
            return;
          }
          const versionNumber = getVersionNumber(version)
            .split('.x')
            .join('.0');
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

  await Promise.all(
    pkgs.map(({ data, path }) => writeJson(path, data, { spaces: indent }))
  );

  _.each(pkgs, (pkg) => {
    console.log(chalk.blue(`./${relative('.', pkg.path)}`));
  });
};
