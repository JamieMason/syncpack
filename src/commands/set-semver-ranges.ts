import { SyncpackConfig } from '../constants';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { setSemverRange as createSetSemverRange } from './lib/set-semver-range';
import { writeIfChanged } from './lib/write-if-changed';
import { getInstallations } from './lib/installations/get-installations';

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
  const { indent, source } = options;
  getWrappers({ source }).forEach((wrapper) => {
    writeIfChanged(indent, wrapper, () => {
      setSemverRanges(wrapper, options);
    });
  });
};
