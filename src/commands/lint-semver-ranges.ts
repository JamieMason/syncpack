import chalk from 'chalk';
import { SyncpackConfig } from '../constants';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { Installation } from './lib/installations/get-dependencies';
import { log } from './lib/log';
import { setSemverRange as createSetSemverRange } from './lib/set-semver-range';
import { getInstallations } from './lib/installations/get-installations';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'peer' | 'prod' | 'semverRange' | 'source'>;

export const lintSemverRanges = (wrappers: SourceWrapper[], options: Options): { installationsWithErrors: Installation[] } => {
  const iterator = getInstallations(wrappers, options);
  const setSemverRange = createSetSemverRange(options);

  const installationsWithErrors: Installation[] = [];

  for(const installation of iterator) {
    const { name, type, version, source } = installation;
    const dependencies = installation.source.contents[type];

    if (dependencies) {
      const currentVersion = dependencies[name];
      const versionWithSelectedSemverRange = setSemverRange(version);
      if(currentVersion !== versionWithSelectedSemverRange) {

        log(chalk`{red ✕ ${name}} ${version} {dim in ${type} of ${source.contents.name}}`)
        installationsWithErrors.push(installation);
      }
    }
  }

  return { installationsWithErrors };
};

export const lintSemverRangesFromDisk = (options: Options): void | never => {
  const wrappers = getWrappers(options);
  const { installationsWithErrors } = lintSemverRanges(wrappers, options);

  if (installationsWithErrors.length > 0) {
    process.exit(1);
  }
};
