import { DependencyType, SyncpackConfig } from '../../../constants';
import { getDependencyTypes } from '../get-dependency-types';
import { SourceWrapper } from '../get-wrappers';
import { getInstallationsOf } from './get-installations-of';

export interface Installation {
  /** which section the package was installed in */
  type: DependencyType;
  /** eg 'lodash' */
  name: string;
  /** package.json file contents */
  source: SourceWrapper;
  /** eg '0.1.0' */
  version: string;
}

export interface InstalledPackage {
  /** eg 'lodash' */
  name: string;
  /** each location this package is installed */
  installations: Installation[];
}

export function* getDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod'>,
): Generator<InstalledPackage> {
  const types = getDependencyTypes(options);
  const visited: { [name: string]: boolean } = {};
  for (const type of types) {
    for (const wrapper of wrappers) {
      if (wrapper.contents[type]) {
        for (const name in wrapper.contents[type]) {
          if (visited[name] === undefined) {
            visited[name] = true;
            yield {
              installations: Array.from(getInstallationsOf(name, types, wrappers)),
              name,
            };
          }
        }
      }
    }
  }
}
