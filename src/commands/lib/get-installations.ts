import { DependencyType } from '../../constants';
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

export function* getDependencies(types: DependencyType[], wrappers: SourceWrapper[]): Generator<InstalledPackage> {
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
  types: DependencyType[],
  wrappers: SourceWrapper[],
): Generator<InstalledPackage> {
  const iterator = getDependencies(types, wrappers);
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
