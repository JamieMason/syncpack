import { isNonEmptyArray, isNonEmptyString, isObject } from 'expect-more';
import minimatch from 'minimatch';
import { verbose } from '../log';
import type { Config, DependencyType } from './get-config/config';
import type { InternalConfig } from './get-config/internal-config';
import type { PackageJsonFile } from './get-package-json-files/package-json-file';

type Group<T> = T & {
  instances: Instance[];
  instancesByName: Record<string, Instance[]>;
  isDefault: boolean;
};

interface Groups {
  semverGroups: SemverGroup.Any[];
  versionGroups: VersionGroup.Any[];
}

export interface Instance {
  dependencyType: DependencyType;
  name: string;
  packageJsonFile: PackageJsonFile;
  version: string;
}

export namespace SemverGroup {
  export type Any = Group<Config.SemverGroup.Any>;
  export type Ignored = Group<Config.SemverGroup.Ignored>;
  export type WithRange = Group<Config.SemverGroup.WithRange>;
}

export namespace VersionGroup {
  export type Any = Group<Config.VersionGroup.Any>;
  export type Standard = Group<Config.VersionGroup.Standard>;
  export type Banned = Group<Config.VersionGroup.Banned>;
  export type Ignored = Group<Config.VersionGroup.Ignored>;
  export type Pinned = Group<Config.VersionGroup.Pinned>;
}

export function getGroups(
  options: InternalConfig,
  packageJsonFiles: PackageJsonFile[],
): Groups {
  const allInstances: Groups = {
    semverGroups: options.semverGroups.map(withInstances),
    versionGroups: options.versionGroups.map(withInstances),
  };

  for (const packageJsonFile of packageJsonFiles) {
    const pkgName = packageJsonFile.contents.name || 'packagewithoutaname';
    for (const dependencyType of options.dependencyTypes) {
      if (dependencyType === 'workspace') {
        const name = packageJsonFile.contents?.name;
        const version = packageJsonFile.contents?.version;
        addInstance({
          dependencyType,
          name,
          pkgName,
          version,
          packageJsonFile: packageJsonFile,
        });
      } else if (dependencyType === 'pnpmOverrides') {
        const versionsByName = packageJsonFile.contents?.pnpm?.overrides;
        if (!isObject<Record<string, string>>(versionsByName)) continue;
        const pkgs = Object.entries(versionsByName);
        for (const [name, version] of pkgs) {
          addInstance({
            dependencyType,
            name,
            pkgName,
            version,
            packageJsonFile: packageJsonFile,
          });
        }
      } else {
        const versionsByName = packageJsonFile.contents?.[dependencyType];
        if (!isObject<Record<string, string>>(versionsByName)) continue;
        const pkgs = Object.entries(versionsByName);
        for (const [name, version] of pkgs) {
          addInstance({
            dependencyType,
            name,
            packageJsonFile,
            pkgName,
            version,
          });
        }
      }
    }
  }

  return allInstances;

  function addInstance(input: {
    dependencyType: DependencyType;
    name?: string;
    packageJsonFile: PackageJsonFile;
    pkgName: string;
    version?: string;
  }): void {
    const { dependencyType, name, packageJsonFile, pkgName, version } = input;
    if (!isNonEmptyString(name)) {
      return verbose('skip instance, no name', input);
    }
    if (name.search(new RegExp(options.filter)) === -1) {
      return verbose('skip instance, name does not match filter', input);
    }
    if (!isNonEmptyString(version)) {
      return verbose('skip instance, no version', input);
    }
    const instance = { dependencyType, name, packageJsonFile, version };
    verbose(
      `add ${name}@${version} to ${dependencyType} ${packageJsonFile.filePath}`,
    );
    groupInstancesBy('semverGroups', dependencyType, pkgName, instance);
    groupInstancesBy('versionGroups', dependencyType, pkgName, instance);
  }

  function withInstances<T>(group: T): Group<T> {
    return {
      ...group,
      instances: [],
      instancesByName: {},
      isDefault:
        group === options.defaultSemverGroup ||
        group === options.defaultVersionGroup,
    };
  }

  function groupInstancesBy(
    groupName: 'semverGroups' | 'versionGroups',
    dependencyType: DependencyType,
    pkgName: string,
    instance: Instance,
  ): void {
    const name = instance.name;
    const groups = allInstances[groupName];
    if (!groups.length) return;
    for (const i in groups) {
      const group = groups[i];
      if (matchesGroup(dependencyType, pkgName, name, group)) {
        if (!group.instancesByName[name]) {
          group.instancesByName[name] = [];
        }
        group.instancesByName[name].push(instance);
        group.instances.push(instance);
        return;
      }
    }
    throw new Error(`${name} in ${pkgName} did not match any ${groupName}`);
  }

  function matchesGroup(
    dependencyType: DependencyType,
    pkgName: string,
    dependencyName: string,
    group: Config.SemverGroup.Any | Config.VersionGroup.Any,
  ): boolean {
    return (
      (!isNonEmptyArray(group.dependencyTypes) ||
        group.dependencyTypes.includes(dependencyType)) &&
      group.packages.some((pattern) => minimatch(pkgName, pattern)) &&
      group.dependencies.some((pattern) => minimatch(dependencyName, pattern))
    );
  }
}
