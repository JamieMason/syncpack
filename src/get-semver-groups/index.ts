import { minimatch } from 'minimatch';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import { getEnabledTypes } from '../config/get-enabled-types';
import { getSemverRange } from '../config/get-semver-range';
import type { Context } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';
import { isValidSemverRange } from '../lib/is-semver';
import { CatchAllSemverGroup } from './catch-all';
import { FilteredOutSemverGroup } from './filtered-out';
import { IgnoredSemverGroup } from './ignored';
import { WithRangeSemverGroup } from './with-range';

export type AnySemverGroup =
  | CatchAllSemverGroup
  | FilteredOutSemverGroup
  | IgnoredSemverGroup
  | WithRangeSemverGroup;

export type SemverGroupReport = {
  name: string;
  instance: Instance;
} & (
  | {
      status: 'FILTERED_OUT';
      isValid: true;
    }
  | {
      status: 'IGNORED';
      isValid: true;
    }
  | {
      status: 'VALID';
      isValid: true;
    }
  | {
      status: 'WORKSPACE_SEMVER_RANGE_MISMATCH';
      isValid: false;
      expectedVersion: string;
    }
  | {
      status: 'SEMVER_RANGE_MISMATCH';
      isValid: false;
      expectedVersion: string;
    }
  | {
      status: 'UNSUPPORTED_VERSION';
      isValid: false;
    }
);

export function getSemverGroups(ctx: Context): AnySemverGroup[] {
  const enabledTypes = getEnabledTypes(ctx.config);
  const semverGroups = createSemverGroups(ctx);

  ctx.packageJsonFiles.forEach((file) => {
    file.getInstances(enabledTypes).forEach((instance) => {
      const { name, pkgName } = instance;
      for (const group of semverGroups) {
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
      throw new Error(`${name} in ${pkgName} did not match any semver groups`);
    });
  });

  return semverGroups.filter((group) => isNonEmptyArray(group.instances));
}

function createSemverGroups(ctx: Context): AnySemverGroup[] {
  const { cli, rcFile } = ctx.config;
  const semverGroups: AnySemverGroup[] = [new FilteredOutSemverGroup(ctx)];

  if (isNonEmptyArray(rcFile.semverGroups)) {
    const ERR_OBJ = new Error('Invalid semverGroup');
    const ERR_DEPS = new Error('Invalid semverGroup dependencies');
    const ERR_PKGS = new Error('Invalid semverGroup packages');

    rcFile.semverGroups.forEach((config) => {
      if (!isObject(config)) throw ERR_OBJ;
      if (!isArrayOfStrings(config.dependencies)) throw ERR_DEPS;
      if (!isArrayOfStrings(config.packages)) throw ERR_PKGS;
      const { dependencies, packages } = config;
      const label = isNonEmptyString(config.label) ? config.label : '';
      const dependencyTypes = isArrayOfStrings(config.dependencyTypes)
        ? config.dependencyTypes
        : [];

      if (config.isIgnored === true) {
        semverGroups.push(
          new IgnoredSemverGroup({
            dependencies,
            dependencyTypes,
            isIgnored: true,
            label,
            packages,
          }),
        );
      } else if (isValidSemverRange(config.range)) {
        semverGroups.push(
          new WithRangeSemverGroup({
            dependencies,
            dependencyTypes,
            label,
            packages,
            range: config.range,
          }),
        );
      }
    });
  }

  semverGroups.push(
    new CatchAllSemverGroup({
      dependencies: ['**'],
      label: '',
      packages: ['**'],
      range: getSemverRange({ cli, rcFile }),
    }),
  );

  return semverGroups;
}
