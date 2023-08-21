import * as Data from '@effect/data/Data';
import * as Option from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { unwrap } from 'tightrope/result/unwrap';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../instance';
import { getHighestVersion } from './lib/get-highest-version';
import { getLowestVersion } from './lib/get-lowest-version';
import { getUniqueSpecifiers } from './lib/get-unique-specifiers';
import { groupBy } from './lib/group-by';

export class StandardVersionGroup extends Data.TaggedClass('Standard')<{
  config: VersionGroupConfig.Standard;
  instances: Instance.Any[];
  isCatchAll: boolean;
}> {
  constructor(isCatchAll: boolean, config: VersionGroupConfig.Standard) {
    super({
      config,
      instances: [],
      isCatchAll,
    });
  }

  canAdd(_: Instance.Any): boolean {
    return true;
  }

  inspect(): Effect.Effect<
    never,
    | VersionGroupReport.LocalPackageMismatch
    | VersionGroupReport.NonSemverMismatch
    | VersionGroupReport.HighestSemverMismatch
    | VersionGroupReport.LowestSemverMismatch,
    VersionGroupReport.Valid
  >[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) => {
      if (!hasMismatch(instances)) {
        return Effect.succeed(
          new VersionGroupReport.Valid({
            name,
            instances,
            isValid: true,
          }),
        );
      }
      const localPackageInstance = getLocalPackageInstance(instances);
      const localPackageFile = localPackageInstance?.packageJsonFile;
      const localPackageVersion = localPackageFile?.contents?.version;
      const isLocalPackage = localPackageInstance && localPackageVersion;
      if (isLocalPackage) {
        return Effect.fail(
          new VersionGroupReport.LocalPackageMismatch({
            name,
            instances,
            isValid: false,
            expectedVersion: localPackageVersion,
            localPackageInstance,
          }),
        );
      }
      if (hasNonSemverSpecifiers(instances)) {
        return Effect.fail(
          new VersionGroupReport.NonSemverMismatch({
            name,
            instances,
            isValid: false,
          }),
        );
      }
      const preferVersion = this.config.preferVersion;
      const expectedVersion = getExpectedVersion(preferVersion, instances);
      return Effect.fail(
        preferVersion === 'highestSemver'
          ? new VersionGroupReport.HighestSemverMismatch({
              name,
              instances,
              isValid: false,
              expectedVersion,
            })
          : new VersionGroupReport.LowestSemverMismatch({
              name,
              instances,
              isValid: false,
              expectedVersion,
            }),
      );
    });
  }
}

function getExpectedVersion(
  preferVersion: VersionGroupConfig.Standard['preferVersion'],
  instances: Instance.Any[],
): string {
  const versions = getUniqueSpecifiers(instances).map((i) => i.specifier);
  return unwrap(
    preferVersion === 'highestSemver' ? getHighestVersion(versions) : getLowestVersion(versions),
  );
}

function hasMismatch(instances: Instance.Any[]): boolean {
  return getUniqueSpecifiers(instances).length > 1;
}

function hasNonSemverSpecifiers(instances: Instance.Any[]): boolean {
  return instances.some((instance) => Option.isNone(instance.getSemverSpecifier()));
}

function getLocalPackageInstance(instances: Instance.Any[]): Instance.Any | undefined {
  return instances.find((instance) => instance.strategy.name === 'local');
}
