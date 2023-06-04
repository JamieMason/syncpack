import minimatch from 'minimatch';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import { getEnabledTypes } from '../config/get-enabled-types';
import type { Context } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';
import { BannedVersionGroup } from './banned';
import { CatchAllVersionGroup } from './catch-all';
import { FilteredOutVersionGroup } from './filtered-out';
import { IgnoredVersionGroup } from './ignored';
import { PinnedVersionGroup } from './pinned';
import { SameRangeVersionGroup } from './same-range';
import { SnappedToVersionGroup } from './snapped-to';
import { StandardVersionGroup } from './standard';

export type AnyVersionGroup =
  | BannedVersionGroup
  | CatchAllVersionGroup
  | FilteredOutVersionGroup
  | IgnoredVersionGroup
  | PinnedVersionGroup
  | SameRangeVersionGroup
  | SnappedToVersionGroup
  | StandardVersionGroup;

export type VersionGroupReport = {
  name: string;
  instances: Instance[];
} & (
  | {
      status: 'BANNED';
      isValid: false;
    }
  | {
      status: 'FILTERED_OUT';
      isValid: true;
    }
  | {
      status: 'HIGHEST_SEMVER_MISMATCH';
      isValid: false;
      expectedVersion: string;
    }
  | {
      status: 'IGNORED';
      isValid: true;
    }
  | {
      status: 'LOWEST_SEMVER_MISMATCH';
      isValid: false;
      expectedVersion: string;
    }
  | {
      status: 'PINNED_MISMATCH';
      isValid: false;
      expectedVersion: string;
    }
  | {
      status: 'SAME_RANGE_MISMATCH';
      isValid: false;
    }
  | {
      status: 'SNAPPED_TO_MISMATCH';
      isValid: false;
      expectedVersion: string;
    }
  | {
      status: 'UNSUPPORTED_MISMATCH';
      isValid: false;
    }
  | {
      status: 'VALID';
      isValid: true;
    }
  | {
      status: 'WORKSPACE_MISMATCH';
      isValid: false;
      expectedVersion: string;
      workspaceInstance: Instance;
    }
);

export function getVersionGroups(ctx: Context): AnyVersionGroup[] {
  const enabledTypes = getEnabledTypes(ctx.config);
  const versionGroups = createVersionGroups(ctx);

  ctx.packageJsonFiles.forEach((file) => {
    file.getInstances(enabledTypes).forEach((instance) => {
      const { name, pkgName } = instance;
      for (const group of versionGroups) {
        const { dependencies, dependencyTypes, packages } = group.config;
        if (
          group.canAdd(instance) &&
          (!isNonEmptyArray(dependencyTypes) ||
            dependencyTypes.includes(instance.strategy.name)) &&
          (!isNonEmptyArray(packages) ||
            packages.some((pattern) => minimatch(instance.pkgName, pattern))) &&
          (!isNonEmptyArray(dependencies) ||
            dependencies.some((pattern) => minimatch(instance.name, pattern)))
        ) {
          group.instances.push(instance);
          return;
        }
      }
      throw new Error(`${name} in ${pkgName} did not match any version groups`);
    });
  });

  return versionGroups.filter((group) => isNonEmptyArray(group.instances));
}

function createVersionGroups(ctx: Context): AnyVersionGroup[] {
  const { rcFile } = ctx.config;
  const versionGroups: AnyVersionGroup[] = [new FilteredOutVersionGroup(ctx)];

  if (isNonEmptyArray(rcFile.versionGroups)) {
    const ERR_OBJ = new Error('Invalid versionGroup');
    const ERR_DEPS = new Error('Invalid versionGroup dependencies');
    const ERR_PKGS = new Error('Invalid versionGroup packages');

    rcFile.versionGroups.forEach((config) => {
      if (!isObject(config)) throw ERR_OBJ;
      if (!isArrayOfStrings(config.dependencies)) throw ERR_DEPS;
      if (!isArrayOfStrings(config.packages)) throw ERR_PKGS;
      const { dependencies, packages } = config;
      const label = isNonEmptyString(config.label) ? config.label : '';
      const dependencyTypes = isArrayOfStrings(config.dependencyTypes)
        ? config.dependencyTypes
        : [];

      if (config.isBanned === true) {
        versionGroups.push(
          new BannedVersionGroup({
            dependencies,
            dependencyTypes,
            isBanned: true,
            label,
            packages,
          }),
        );
      } else if (config.isIgnored === true) {
        versionGroups.push(
          new IgnoredVersionGroup({
            dependencies,
            dependencyTypes,
            isIgnored: true,
            label,
            packages,
          }),
        );
      } else if (isNonEmptyString(config.pinVersion)) {
        versionGroups.push(
          new PinnedVersionGroup({
            dependencies,
            dependencyTypes,
            label,
            packages,
            pinVersion: config.pinVersion,
          }),
        );
      } else if (isArrayOfStrings(config.snapTo)) {
        versionGroups.push(
          new SnappedToVersionGroup({
            dependencies,
            dependencyTypes,
            label,
            packages,
            snapTo: config.snapTo,
          }),
        );
      } else if (config.policy === 'sameRange') {
        versionGroups.push(
          new SameRangeVersionGroup({
            dependencies,
            dependencyTypes,
            label,
            packages,
            policy: config.policy,
          }),
        );
      } else {
        versionGroups.push(
          new StandardVersionGroup({
            dependencies,
            dependencyTypes,
            label,
            packages,
            preferVersion:
              config.preferVersion === 'lowestSemver'
                ? 'lowestSemver'
                : 'highestSemver',
          }),
        );
      }
    });
  }

  versionGroups.push(
    new CatchAllVersionGroup({
      dependencies: ['**'],
      packages: ['**'],
      preferVersion: 'highestSemver',
    }),
  );

  return versionGroups;
}
