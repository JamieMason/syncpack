import chalk from 'chalk';
import { ICON } from '../constants';
import type { Context } from '../get-context';
import { getSemverGroups } from '../get-semver-groups';
import { getVersionGroups } from '../get-version-groups';

export function lint(ctx: Context): Context {
  let totalInstances = 0;
  const instanceCounts: Record<string, number> = {};
  const versionGroups = getVersionGroups(ctx);
  const versionCounts: Record<string, number> = {
    BANNED: 0,
    FILTERED_OUT: 0,
    HIGHEST_SEMVER_MISMATCH: 0,
    IGNORED: 0,
    LOWEST_SEMVER_MISMATCH: 0,
    PINNED_MISMATCH: 0,
    SEMVER_UNSATISFIED: 0,
    SNAPPED_TO_MISMATCH: 0,
    UNSUPPORTED_MISMATCH: 0,
    VALID: 0,
    WORKSPACE_MISMATCH: 0,
    WORKSPACE_UNSATISFIED: 0,
  };

  versionGroups.forEach((group) => {
    group.inspect().forEach((report) => {
      if (!report.isValid) ctx.isInvalid = true;
      if (!versionCounts[report.status]) {
        versionCounts[report.status] = 0;
      }
      versionCounts[report.status]++;
    });
  });

  const semverGroups = getSemverGroups(ctx);
  const semverCounts: Record<string, number> = {
    FILTERED_OUT: 0,
    IGNORED: 0,
    VALID: 0,
    WORKSPACE_SEMVER_RANGE_MISMATCH: 0,
    SEMVER_RANGE_MISMATCH: 0,
    UNSUPPORTED_VERSION: 0,
  };

  semverGroups.forEach((group) => {
    group.inspect().forEach((report) => {
      const status = report.status;
      const strategyName = report.instance.strategy.name;

      if (!report.isValid) ctx.isInvalid = true;

      if (!semverCounts[status]) semverCounts[status] = 0;
      semverCounts[status]++;

      if (!instanceCounts[strategyName]) instanceCounts[strategyName] = 0;
      instanceCounts[strategyName]++;

      totalInstances++;
    });
  });

  const fileCount = ctx.packageJsonFiles.length;
  info(fileCount, `package.json file${fileCount > 1 ? 's' : ''}`);
  info(totalInstances, 'total dependences');

  Object.entries(instanceCounts).forEach(([name, count]) => {
    info(
      count,
      `in package.${name
        .replace(/^dev$/, 'devDependencies')
        .replace(/^peer$/, 'peerDependencies')
        .replace(/^pnpmOverrides$/, 'pnpm.overrides')
        .replace(/^prod$/, 'dependencies')
        .replace(/^workspace$/, 'version')}`,
    );
  });

  console.log(chalk.blue('Version mismatches'));

  expectSome(versionCounts.VALID, 'are valid');
  expectNone(
    versionCounts.HIGHEST_SEMVER_MISMATCH,
    'should have matching versions (prefer highest semver)',
  );
  expectNone(
    versionCounts.LOWEST_SEMVER_MISMATCH,
    'should have matching versions (prefer lowest semver)',
  );
  expectNone(
    versionCounts.WORKSPACE_MISMATCH,
    'should match workspace package version',
  );
  expectNone(versionCounts.BANNED, 'are banned from use');
  expectNone(
    versionCounts.PINNED_MISMATCH,
    'should be pinned to a specific version',
  );
  expectNone(
    versionCounts.SNAPPED_TO_MISMATCH,
    'should snap to version used by another package',
  );
  info(versionCounts.FILTERED_OUT, 'are filtered out');
  info(versionCounts.IGNORED, 'are ignored');
  info(versionCounts.UNSUPPORTED_MISMATCH, 'have unsupported version format');

  console.log(chalk.blue('Semver range mismatches'));

  expectSome(semverCounts.VALID, 'are valid');
  expectNone(
    semverCounts.SEMVER_RANGE_MISMATCH,
    'should use specific semver range',
  );
  expectNone(
    semverCounts.WORKSPACE_SEMVER_RANGE_MISMATCH,
    'should use exact version in version property of package.json',
  );
  info(semverCounts.FILTERED_OUT, 'are filtered out');
  info(semverCounts.IGNORED, 'are ignored');
  info(semverCounts.UNSUPPORTED_VERSION, 'have unsupported version format');

  return ctx;

  function expectNone(count = 0, ...rest: string[]) {
    if (count === 0) {
      console.log(chalk.dim(ICON.skip), pad(count), ...rest);
    } else {
      console.log(chalk.red(ICON.cross, pad(count), ...rest));
    }
  }

  function expectSome(count = 0, ...rest: string[]) {
    if (count > 0) {
      console.log(chalk.dim(ICON.skip), pad(count), ...rest);
    } else {
      console.log(chalk.red(ICON.cross, pad(count), ...rest));
    }
  }

  function info(count = 0, ...rest: string[]) {
    console.log(chalk.dim(ICON.skip), pad(count), ...rest);
  }

  function pad(count: number): any {
    return String(count).padStart(String(totalInstances).length, ' ');
  }
}
