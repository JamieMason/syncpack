import chalk from 'chalk';
import { ICON } from '../constants';
import type { Context } from '../lib/get-context';
import type { InstanceGroup } from '../lib/get-context/get-groups/version-group/instance-group';
import type { Instance } from '../lib/get-context/get-package-json-files/package-json-file/instance';
import { logVersionGroupHeader } from '../lib/log';

export function listMismatches(ctx: Context): Context {
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    const invalidGroups = versionGroup.getInvalidInstanceGroups();

    // Nothing to do if there are no mismatches
    if (invalidGroups.length === 0) return;

    // Record that this project has mismatches, so that eg. the CLI can exit
    // with the correct status code.
    ctx.isInvalid = true;

    // Annotate user-defined version groups
    if (!versionGroup.isDefault) logVersionGroupHeader(i);

    // Log the mismatches
    invalidGroups.forEach((instanceGroup) => {
      const name = instanceGroup.name;
      const workspaceInstance = instanceGroup.getWorkspaceInstance();
      const expected = instanceGroup.getExpectedVersion();
      const isBanned = instanceGroup.versionGroup.isBanned;
      const isUnpinned = instanceGroup.isUnpinned;

      if (isBanned) {
        logBanned(name);
      } else if (isUnpinned) {
        logPinVersionMismatch(name, instanceGroup);
      } else if (workspaceInstance) {
        logWorkspaceMismatch(workspaceInstance, expected, name);
      } else {
        logHighestVersionMismatch(expected, name);
      }

      instanceGroup.instances.forEach((instance) => {
        if (instance.version === expected) {
          logVersionMatch(instance);
        } else {
          logVersionMismatch(instance);
        }
      });
    });
  });

  return ctx;

  function logVersionMatch(instance: Instance): void {
    const { dependencyType, version, packageJsonFile } = instance;
    const isWorkspace = dependencyType === 'workspace';
    const shortPath = packageJsonFile.shortPath;
    const loc = isWorkspace ? 'version' : dependencyType;
    console.log(chalk`{green   ${version} in ${loc} of ${shortPath}}`);
  }

  function logVersionMismatch(instance: Instance): void {
    const { dependencyType, version, packageJsonFile } = instance;
    const isWorkspace = dependencyType === 'workspace';
    const shortPath = packageJsonFile.shortPath;
    const loc = isWorkspace ? 'version' : dependencyType;
    console.log(chalk`{red   ${version} in ${loc} of ${shortPath}}`);
  }

  function logHighestVersionMismatch(
    expected: string | undefined,
    name: string,
  ) {
    const reason = chalk`{dim : ${expected} is the highest valid semver version in use}`;
    console.log(chalk`{dim -} ${name}${reason}`);
  }

  function logWorkspaceMismatch(
    workspaceInstance: Instance,
    expected: string | undefined,
    name: string,
  ) {
    const shortPath = workspaceInstance.packageJsonFile.shortPath;
    const reason = chalk`{dim : ${expected} is developed in this repo at ${shortPath}}`;
    console.log(chalk`{dim -} ${name}${reason}`);
  }

  function logPinVersionMismatch(name: string, instanceGroup: InstanceGroup) {
    console.log(
      chalk`{red ${ICON.cross} ${name}} {dim.red is defined in this version group to be pinned at ${instanceGroup.versionGroup.pinVersion}}`,
    );
  }

  function logBanned(name: string) {
    console.log(
      chalk`{red ${ICON.cross} ${name}} {dim.red is defined in this version group as banned from use}`,
    );
  }
}
