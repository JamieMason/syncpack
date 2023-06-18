import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { unwrap } from 'tightrope/result/unwrap';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { isSupported } from '../guards/is-supported';
import { getHighestVersion } from './lib/get-highest-version';
import { getLowestVersion } from './lib/get-lowest-version';
import { getUniqueVersions } from './lib/get-unique-versions';
import { groupBy } from './lib/group-by';

export class StandardVersionGroup extends Data.TaggedClass('Standard')<{
  config: VersionGroupConfig.Standard;
  instances: Instance[];
  isCatchAll: boolean;
}> {
  constructor(isCatchAll: boolean, config: VersionGroupConfig.Standard) {
    super({
      config,
      instances: [],
      isCatchAll,
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): Effect.Effect<
    never,
    | VersionGroupReport.WorkspaceMismatch
    | VersionGroupReport.UnsupportedMismatch
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
      const wsInstance = getWorkspaceInstance(instances);
      const wsFile = wsInstance?.packageJsonFile;
      const wsVersion = wsFile?.contents?.version;
      const isWorkspacePackage = wsInstance && wsVersion;
      if (isWorkspacePackage) {
        const nonWsInstances = getNonWorkspaceInstances(instances);
        if (!hasMismatch(nonWsInstances)) {
          return Effect.succeed(
            new VersionGroupReport.Valid({
              name,
              instances,
              isValid: true,
            }),
          );
        }
        return Effect.fail(
          new VersionGroupReport.WorkspaceMismatch({
            name,
            instances,
            isValid: false,
            expectedVersion: wsVersion,
            workspaceInstance: wsInstance,
          }),
        );
      }
      if (hasUnsupported(instances)) {
        return Effect.fail(
          new VersionGroupReport.UnsupportedMismatch({
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
  instances: Instance[],
): string {
  const versions = getUniqueVersions(instances);
  return unwrap(
    preferVersion === 'highestSemver' ? getHighestVersion(versions) : getLowestVersion(versions),
  );
}

function hasMismatch(instances: Instance[]): boolean {
  return getUniqueVersions(instances).length > 1;
}

function hasUnsupported(instances: Instance[]): boolean {
  return instances.some((instance) => !isSupported(instance.version));
}

function getWorkspaceInstance(instances: Instance[]): Instance | undefined {
  return instances.find((instance) => instance.strategy.name === 'workspace');
}

function getNonWorkspaceInstances(instances: Instance[]) {
  return instances.filter((instance) => instance.strategy.name !== 'workspace');
}
