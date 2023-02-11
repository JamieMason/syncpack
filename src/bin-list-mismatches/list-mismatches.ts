import chalk from 'chalk';
import type { VersionGroup } from '../get-context/get-groups/version-group';
import type { Instance } from '../get-context/get-package-json-files/package-json-file/instance';
import * as log from '../lib/log';
import type { Syncpack } from '../types';

export function listMismatches(ctx: Syncpack.Ctx): Syncpack.Ctx {
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    const invalidGroups = versionGroup.getInvalidInstanceGroups();

    // Nothing to do if there are no mismatches
    if (invalidGroups.length === 0) return;

    // Record that this project has mismatches, so that eg. the CLI can exit
    // with the correct status code.
    ctx.isInvalid = true;

    // Annotate user-defined version groups
    if (!versionGroup.isDefault) log.versionGroupHeader(i);

    // Log the mismatches
    invalidGroups.forEach((instanceGroup) => {
      const name = instanceGroup.name;
      const workspaceInstance = instanceGroup.getWorkspaceInstance();
      const expected = instanceGroup.getExpectedVersion() || '';
      const isBanned = versionGroup.isBanned;
      const isUnpinned = instanceGroup.isUnpinned;

      // Log the dependency name
      if (isBanned) {
        logBanned(name);
      } else if (isUnpinned) {
        logPinVersionMismatch(name, versionGroup);
      } else if (workspaceInstance) {
        logWorkspaceMismatch(workspaceInstance, expected, name);
      } else {
        logHighestVersionMismatch(expected, name);
      }

      // Log each of the dependencies mismatches
      instanceGroup.instances.forEach((instance) => {
        if (instance.version !== expected) {
          logVersionMismatch(instance);
        }
      });
    });
  });

  return ctx;

  function logBanned(name: string) {
    log.invalid(name, 'is banned in this version group');
  }

  function logPinVersionMismatch(name: string, versionGroup: VersionGroup) {
    const pinVersion = versionGroup.pinVersion;
    log.invalid(
      name,
      chalk`is pinned in this version group at {reset.green ${pinVersion}}`,
    );
  }

  function logWorkspaceMismatch(
    workspaceInstance: Instance,
    expected: string | undefined,
    name: string,
  ) {
    const shortPath = workspaceInstance.packageJsonFile.shortPath;
    log.invalid(
      name,
      chalk`{reset.green ${expected}} {dim is developed in this repo at ${shortPath}}`,
    );
  }

  function logHighestVersionMismatch(
    expected: string | undefined,
    name: string,
  ) {
    log.invalid(
      name,
      chalk`{reset.green ${expected}} {dim is the highest valid semver version in use}`,
    );
  }

  function logVersionMismatch(instance: Instance): void {
    const type = instance.dependencyType;
    const shortPath = instance.packageJsonFile.shortPath;
    const actual = instance.version;
    console.log(chalk`  {red ${actual}} {dim in ${type} of ${shortPath}}`);
  }
}
