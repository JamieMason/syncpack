import { IManifestDescriptor, IManifestKey } from '../typings';

export interface IVersionsByName {
  [name: string]: string[];
}

export type GetVersionsByName = (
  dependencyTypes: IManifestKey[],
  pkgs: IManifestDescriptor[],
  dependencyFilterPattern?: string
) => IVersionsByName;

export const getDependencyFilter = (dependencyFilterPattern?: string) => {
  if (!dependencyFilterPattern) {
    return () => true;
  }
  const dependencyMatcher = new RegExp(dependencyFilterPattern);

  return (dependencyName: string) => dependencyMatcher.test(dependencyName);
};

export const getVersionsByName: GetVersionsByName = (
  dependencyTypes,
  pkgs,
  dependencyFilterPattern
) => {
  const dependencyFilter = getDependencyFilter(dependencyFilterPattern);
  const versionsByName: IVersionsByName = {};
  for (const type of dependencyTypes) {
    for (const pkg of pkgs) {
      const dependencies = pkg.data[type];
      if (dependencies) {
        for (const name in dependencies) {
          if (!dependencyFilter(name)) {
            continue;
          }
          if (dependencies.hasOwnProperty(name)) {
            const version = dependencies[name];
            versionsByName[name] = versionsByName[name] || [];
            if (!versionsByName[name].includes(version)) {
              versionsByName[name].push(version);
            }
          }
        }
      }
    }
  }
  return versionsByName;
};

export const getMismatchedVersionsByName: GetVersionsByName = (
  dependencyTypes,
  pkgs,
  dependencyFilterPattern
) => {
  const dependencyFilter = getDependencyFilter(dependencyFilterPattern);
  const mismatchedVersionsByName: IVersionsByName = {};
  const versionsByName = getVersionsByName(
    dependencyTypes,
    pkgs,
    dependencyFilterPattern
  );
  for (const type of dependencyTypes) {
    for (const pkg of pkgs) {
      const dependencies = pkg.data[type];
      if (dependencies) {
        for (const name in dependencies) {
          if (!dependencyFilter(name)) {
            continue;
          }
          if (dependencies.hasOwnProperty(name)) {
            if (versionsByName[name].length > 1) {
              mismatchedVersionsByName[name] = versionsByName[name];
            }
          }
        }
      }
    }
  }
  return mismatchedVersionsByName;
};
