import { isNonEmptyArray, isNonEmptyString, isObject } from 'expect-more';
import minimatch from 'minimatch';
import type {
  DependencyType,
  SemverGroup,
  SyncpackConfig,
  VersionGroup,
} from '../../constants';
import type { SourceWrapper } from './get-wrappers';

export interface Instance {
  dependencyType: DependencyType;
  name: string;
  version: string;
  wrapper: SourceWrapper;
}

export interface InstanceIndex {
  instances: Instance[];
  instancesByName: InstancesByName;
}

export type InstancesByName = Record<string, Instance[]>;
export type IndexedSemverGroup = SemverGroup & InstanceIndex;
export type IndexedVersionGroup = VersionGroup & InstanceIndex;

export interface Instances {
  all: Instance[];
  semverGroups: IndexedSemverGroup[];
  versionGroups: IndexedVersionGroup[];
}

export function getInstances(
  options: SyncpackConfig,
  wrappers: SourceWrapper[],
): Instances {
  const allInstances: Instances = {
    all: [],
    semverGroups: options.semverGroups.map(withInstances),
    versionGroups: options.versionGroups.map(withInstances),
  };

  for (const wrapper of wrappers) {
    const pkgName = wrapper.contents.name || 'packagewithoutaname';
    for (const dependencyType of options.dependencyTypes) {
      if (dependencyType === 'workspace') {
        const name = wrapper.contents?.name;
        const version = wrapper.contents?.version;
        if (!isNonEmptyString(name)) continue;
        if (name.search(new RegExp(options.filter)) === -1) continue;
        if (!isNonEmptyString(version)) continue;
        const instance = { dependencyType, name, version, wrapper };
        allInstances.all.push(instance);
        groupInstancesBy('semverGroups', dependencyType, pkgName, instance);
        groupInstancesBy('versionGroups', dependencyType, pkgName, instance);
      } else {
        const versionsByName = wrapper.contents?.[dependencyType];
        if (!isObject<Record<string, string>>(versionsByName)) continue;
        const pkgs = Object.entries(versionsByName);
        for (const [name, version] of pkgs) {
          if (!isNonEmptyString(name)) continue;
          if (name.search(new RegExp(options.filter)) === -1) continue;
          if (!isNonEmptyString(version)) continue;
          const instance = { dependencyType, name, version, wrapper };
          allInstances.all.push(instance);
          groupInstancesBy('semverGroups', dependencyType, pkgName, instance);
          groupInstancesBy('versionGroups', dependencyType, pkgName, instance);
        }
      }
    }
  }

  return allInstances;

  function withInstances<T>(group: T): T & InstanceIndex {
    const instances: InstanceIndex['instances'] = [];
    const instancesByName: InstanceIndex['instancesByName'] = {};
    return { ...group, instances, instancesByName };
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
    group: SemverGroup | VersionGroup,
  ): boolean {
    return (
      (!isNonEmptyArray(group.dependencyTypes) ||
        group.dependencyTypes.includes(dependencyType)) &&
      group.packages.some((pattern) => minimatch(pkgName, pattern)) &&
      group.dependencies.some((pattern) => minimatch(dependencyName, pattern))
    );
  }
}
