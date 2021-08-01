import { SyncpackConfig } from '../constants';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { getInstallations } from './lib/installations/get-installations';
import { setSemverRange as createSetSemverRange } from './lib/set-semver-range';
import { writeIfChanged } from './lib/write-if-changed';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'indent' | 'peer' | 'prod' | 'semverRange' | 'source'>;

export const setSemverRanges = (wrapper: SourceWrapper, options: Options): void => {
  const installationsIterator = getInstallations([wrapper], options);
  const setSemverRange = createSetSemverRange(options);

  for (const installation of installationsIterator) {
    const { name, type, version } = installation;
    const dependencies = installation.source.contents[type];
    if (dependencies) {
      dependencies[name] = setSemverRange(version);
    }
  }
};

export const setSemverRangesToDisk = (options: Options): void => {
  getWrappers({ source: options.source }).forEach((wrapper) => {
    writeIfChanged(options.indent, wrapper, () => {
      setSemverRanges(wrapper, options);
    });
  });
};
