import { isNonEmptyString } from 'expect-more';
import type { Config } from './get-config/config';
import type { InternalConfig } from './get-config/internal-config';
import type { Instance } from './get-package-json-files/package-json-file/instance';

type Group<T> = T & {
  instances: Instance[];
  instancesByName: Record<string, Instance[]>;
  isDefault: boolean;
};

export namespace SemverGroup {
  export type Any = Group<Config.SemverGroup.Any>;
  export type Ignored = Group<Config.SemverGroup.Ignored>;
  export type WithRange = Group<Config.SemverGroup.WithRange>;
}

export function getSemverGroups(
  input: InternalConfig,
  instances: Instance[],
): SemverGroup.Any[] {
  const semverGroups = input.semverGroups.map(
    (semverGroup): SemverGroup.Any => ({
      ...semverGroup,
      instances: [],
      instancesByName: {},
      isDefault: semverGroup === input.defaultSemverGroup,
    }),
  );

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    for (const semverGroup of semverGroups) {
      if (instance.matchesGroup(semverGroup)) {
        if (!semverGroup.instancesByName[name]) {
          semverGroup.instancesByName[name] = [];
        }
        semverGroup.instancesByName[name].push(instance);
        semverGroup.instances.push(instance);
        return;
      }
    }
    throw new Error(`${name} in ${pkgName} did not match any semverGroups`);
  });

  return semverGroups;
}

export namespace VersionGroup {
  export interface InstanceGroup {
    hasMismatches: boolean;
    instances: Instance[];
    isBanned: boolean;
    isIgnored: boolean;
    isInvalid: boolean;
    name: string;
    uniques: string[];
  }

  type Base<T> = Group<T> & { instanceGroups: InstanceGroup[] };

  export type Any = Base<Config.VersionGroup.Any>;
  export type Standard = Base<Config.VersionGroup.Standard>;
  export type Banned = Base<Config.VersionGroup.Banned>;
  export type Ignored = Base<Config.VersionGroup.Ignored>;
  export type Pinned = Base<Config.VersionGroup.Pinned>;
}

export function getVersionGroups(
  input: InternalConfig,
  instances: Instance[],
): VersionGroup.Any[] {
  const versionGroups = input.versionGroups.map(
    (versionGroup): VersionGroup.Any => ({
      ...versionGroup,
      instanceGroups: [],
      instances: [],
      instancesByName: {},
      isDefault: versionGroup === input.defaultVersionGroup,
    }),
  );

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    for (const versionGroup of versionGroups) {
      if (instance.matchesGroup(versionGroup)) {
        if (!versionGroup.instancesByName[name]) {
          versionGroup.instancesByName[name] = [];
        }
        versionGroup.instancesByName[name].push(instance);
        versionGroup.instances.push(instance);
        return;
      }
    }
    throw new Error(`${name} in ${pkgName} did not match any versionGroups`);
  });

  versionGroups.forEach((versionGroup) => {
    versionGroup.instanceGroups = getInstanceGroups(versionGroup);
  });

  return versionGroups;
}

function getInstanceGroups(
  versionGroup: VersionGroup.Any,
): VersionGroup.InstanceGroup[] {
  return Object.entries(versionGroup.instancesByName).map(
    ([name, instances]): VersionGroup.InstanceGroup => {
      const pinnedVersion = (versionGroup as VersionGroup.Pinned).pinVersion;
      const isBanned = (versionGroup as VersionGroup.Banned).isBanned === true;
      const isIgnored =
        (versionGroup as VersionGroup.Ignored).isIgnored === true;
      const hasPinnedVersion = isNonEmptyString(pinnedVersion);
      const versions = instances.map(({ version }) => version);
      const uniques = Array.from(new Set(versions));
      const [version] = uniques;
      const isUnpinned = hasPinnedVersion && version !== pinnedVersion;
      const hasMismatches = isBanned || isUnpinned || uniques.length > 1;
      const isInvalid = !isIgnored && hasMismatches;
      return {
        hasMismatches,
        instances,
        isBanned,
        isIgnored,
        isInvalid,
        name,
        uniques,
      };
    },
  );
}
