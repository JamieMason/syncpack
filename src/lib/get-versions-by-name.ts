import { IManifestDescriptor, IManifestKey } from '../typings';

export interface IVersionsByName {
  [name: string]: string[];
}

export type GetVersionsByName = (
  dependencyTypes: IManifestKey[],
  pkgs: IManifestDescriptor[]
) => IVersionsByName;

export const getVersionsByName: GetVersionsByName = (dependencyTypes, pkgs) => {
  const versionsByName: IVersionsByName = {};
  for (const type of dependencyTypes) {
    for (const pkg of pkgs) {
      const dependencies = pkg.data[type];
      if (dependencies) {
        for (const name in dependencies) {
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
  pkgs
) => {
  const mismatchedVersionsByName: IVersionsByName = {};
  const versionsByName = getVersionsByName(dependencyTypes, pkgs);
  for (const type of dependencyTypes) {
    for (const pkg of pkgs) {
      const dependencies = pkg.data[type];
      if (dependencies) {
        for (const name in dependencies) {
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
