import chalk from 'chalk';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { ICON } from '../constants';
import type { Context } from '../get-context';
import { getVersionGroups } from '../get-version-groups';
import type { SnappedToVersionGroup } from '../get-version-groups/snapped-to';
import * as log from '../lib/log';
import { sortByName } from '../lib/sort-by-name';

export function listMismatches(ctx: Context): Context {
  const versionGroups = getVersionGroups(ctx);
  const hasUserGroups = isNonEmptyArray(ctx.config.rcFile.versionGroups);

  versionGroups.forEach((versionGroup, i) => {
    versionGroup
      .inspect()
      .sort(sortByName)
      .forEach((report, ii) => {
        // no action needed
        if (report.isValid) return;

        // Allow eg. CLI to exit with the correct status code.
        ctx.isInvalid = true;

        // Annotate each group
        if (ii === 0 && hasUserGroups) log.versionGroupHeader(versionGroup, i);

        switch (report.status) {
          case 'BANNED':
            console.log(
              chalk`  {red %s} %s {dim is banned in this version group}`,
              ICON.cross,
              report.name,
            );
            report.instances.forEach((instance) => {
              console.log(
                chalk`  {red %s} {dim in %s of %s}`,
                instance.version,
                instance.strategy.path,
                instance.packageJsonFile.shortPath,
              );
            });
            break;
          case 'HIGHEST_SEMVER_MISMATCH':
          case 'LOWEST_SEMVER_MISMATCH': {
            console.log(
              chalk`{red %s} %s {green %s} {dim is the %s valid semver version in use}`,
              ICON.cross,
              report.name,
              report.expectedVersion,
              report.status === 'LOWEST_SEMVER_MISMATCH' ? 'lowest' : 'highest',
            );
            report.instances.forEach((instance) => {
              if (instance.version !== report.expectedVersion) {
                console.log(
                  chalk`  {red %s} {dim in %s of %s}`,
                  instance.version,
                  instance.strategy.path,
                  instance.packageJsonFile.shortPath,
                );
              }
            });
            break;
          }
          case 'PINNED_MISMATCH': {
            console.log(
              chalk`{red %s} %s {dim is pinned in this version group at} {green %s}`,
              ICON.cross,
              report.name,
              report.expectedVersion,
            );
            report.instances.forEach((instance) => {
              if (instance.version !== report.expectedVersion) {
                console.log(
                  chalk`  {red %s} {dim in %s of %s}`,
                  instance.version,
                  instance.strategy.path,
                  instance.packageJsonFile.shortPath,
                );
              }
            });
            break;
          }
          case 'SNAPPED_TO_MISMATCH': {
            console.log(
              chalk`{red %s} %s {dim should snap to {reset.green %s}, used by %s}`,
              ICON.cross,
              report.name,
              report.expectedVersion,
              (versionGroup as SnappedToVersionGroup).config.snapTo.join(
                ' || ',
              ),
            );
            report.instances.forEach((instance) => {
              if (instance.version !== report.expectedVersion) {
                console.log(
                  chalk`  {red %s} {dim in %s of %s}`,
                  instance.version,
                  instance.strategy.path,
                  instance.packageJsonFile.shortPath,
                );
              }
            });
            break;
          }
          case 'SAME_RANGE_MISMATCH': {
            console.log(
              chalk`{red %s} %s {dim has mismatched semver range versions which syncpack cannot fix}`,
              ICON.cross,
              report.name,
            );
            report.instances.forEach((instance) => {
              console.log(
                chalk`  {yellow %s} {dim in %s of %s}`,
                instance.version,
                instance.strategy.path,
                instance.packageJsonFile.shortPath,
              );
            });
            break;
          }
          case 'UNSUPPORTED_MISMATCH': {
            console.log(
              chalk`{red %s} %s {dim has mismatched versions which syncpack cannot fix}`,
              ICON.cross,
              report.name,
            );
            report.instances.forEach((instance) => {
              console.log(
                chalk`  {yellow %s} {dim in %s of %s}`,
                instance.version,
                instance.strategy.path,
                instance.packageJsonFile.shortPath,
              );
            });
            break;
          }
          case 'WORKSPACE_MISMATCH': {
            console.log(
              chalk`{red %s} %s {green %s} {dim is developed in this repo at %s}`,
              ICON.cross,
              report.name,
              report.expectedVersion,
              report.workspaceInstance.packageJsonFile.shortPath,
            );
            report.instances.forEach((instance) => {
              if (instance.version !== report.expectedVersion) {
                console.log(
                  chalk`  {red %s} {dim in %s of %s}`,
                  instance.version,
                  instance.strategy.path,
                  instance.packageJsonFile.shortPath,
                );
              }
            });
            break;
          }
        }
      });
  });

  return ctx;
}
