import { DependencyType, SyncpackConfig } from '../../constants';
import { getDependencyTypes } from './get-dependency-types';
import { SourceWrapper } from './get-wrappers';

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

function* getInstallationsOf(
  name: string,
  types: DependencyType[],
  wrappers: SourceWrapper[],
): Generator<Installation> {
  for (const type of types) {
    for (const wrapper of wrappers) {
      const dependencies = wrapper.contents[type];
      if (dependencies && dependencies[name]) {
        yield {
          name,
          source: wrapper,
          type,
          version: dependencies[name],
        };
      }
    }
  }
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

export function* getMismatchedDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod'>,
): Generator<InstalledPackage> {
  const iterator = getDependencies(wrappers, options);
  for (const installedPackage of iterator) {
    const { installations } = installedPackage;
    const len = installations.length;
    if (len > 1) {
      for (let i = 1; i < len; i++) {
        if (installations[i].version !== installations[i - 1].version) {
          yield installedPackage;
          break;
        }
      }
    }
  }
}

export const sortByName = (a: InstalledPackage, b: InstalledPackage): 0 | 1 | -1 => {
  if (a.name < b.name) {
    return -1;
  }
  if (a.name > b.name) {
    return 1;
  }
  return 0;
};
