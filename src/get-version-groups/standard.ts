import { unwrap } from 'tightrope/result/unwrap';
import type { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { isSupported } from '../lib/is-semver';
import { getHighestVersion } from './lib/get-highest-version';
import { getLowestVersion } from './lib/get-lowest-version';
import { getUniqueVersions } from './lib/get-unique-versions';
import { groupBy } from './lib/group-by';

export class StandardVersionGroup {
  _tag = 'Standard';
  config: VersionGroupConfig.Standard;
  instances: Instance[];

  constructor(config: VersionGroupConfig.Standard) {
    this.config = config;
    this.instances = [];
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): VersionGroupReport[] {
    const report: VersionGroupReport[] = [];
    const instancesByName = groupBy('name', this.instances);

    Object.entries(instancesByName).forEach(([name, instances]) => {
      if (!hasMismatch(instances)) {
        return report.push({
          status: 'VALID',
          instances,
          isValid: true,
          name,
        });
      }
      const wsInstance = getWorkspaceInstance(instances);
      const wsFile = wsInstance?.packageJsonFile;
      const wsVersion = wsFile?.contents?.version;
      const isWorkspacePackage = wsInstance && wsVersion;
      if (isWorkspacePackage) {
        const nonWsInstances = getNonWorkspaceInstances(instances);
        if (!hasMismatch(nonWsInstances)) {
          return report.push({
            status: 'VALID',
            instances,
            isValid: true,
            name,
          });
        }
        return report.push({
          status: 'WORKSPACE_MISMATCH',
          expectedVersion: wsVersion,
          instances,
          isValid: false,
          name,
          workspaceInstance: wsInstance,
        });
      }
      if (hasUnsupported(instances)) {
        return report.push({
          status: 'UNSUPPORTED_MISMATCH',
          instances,
          isValid: false,
          name,
        });
      }
      const preferVersion = this.config.preferVersion;
      const expectedVersion = getExpectedVersion(preferVersion, instances);
      return report.push({
        status:
          preferVersion === 'highestSemver'
            ? 'HIGHEST_SEMVER_MISMATCH'
            : 'LOWEST_SEMVER_MISMATCH',
        expectedVersion,
        instances,
        isValid: false,
        name,
      });
    });

    return report;
  }
}

function getExpectedVersion(
  preferVersion: VersionGroupConfig.Standard['preferVersion'],
  instances: Instance[],
): string {
  const versions = getUniqueVersions(instances);
  return unwrap(
    preferVersion === 'highestSemver'
      ? getHighestVersion(versions)
      : getLowestVersion(versions),
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
