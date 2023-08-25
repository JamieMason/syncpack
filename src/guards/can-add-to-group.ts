import { minimatch } from 'minimatch';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import type { Ctx } from '../get-context';
import type { Instance } from '../get-instances/instance';
import type { SemverGroup } from '../semver-group';
import type { VersionGroup } from '../version-group';

export function canAddToGroup(
  packageJsonFilesByName: Ctx['packageJsonFilesByName'],
  group: SemverGroup.Any | VersionGroup.Any,
  instance: Instance,
): boolean {
  const { dependencies, dependencyTypes, packages } = group.config;
  return (
    group.canAdd(instance) &&
    matchesDependencyTypes(dependencyTypes, instance) &&
    matchesPackages(packages, instance) &&
    matchesDependencies(packageJsonFilesByName, group, dependencies, instance)
  );
}

function matchesDependencies(
  packageJsonFilesByName: Ctx['packageJsonFilesByName'],
  group: SemverGroup.Any | VersionGroup.Any,
  dependencies: unknown,
  instance: Instance,
): boolean {
  // matches if not defined
  if (!isNonEmptyArray(dependencies)) return true;
  return dependencies.some(
    (pattern) =>
      (pattern === '$LOCAL' &&
        instance.name in packageJsonFilesByName &&
        ((group.groupType === 'versionGroup' && instance.versionGroup === null) ||
          (group.groupType === 'semverGroup' && instance.semverGroup === null))) ||
      minimatch(instance.name, pattern),
  );
}

function matchesPackages(packages: unknown, instance: Instance) {
  // matches if not defined
  if (!isNonEmptyArray(packages)) return true;
  return packages.some((pattern) => minimatch(instance.pkgName, pattern));
}

function matchesDependencyTypes(dependencyTypes: unknown, instance: Instance) {
  // matches if not defined
  if (!isNonEmptyArray(dependencyTypes)) return true;
  if (dependencyTypes.join('') === '**') return true;
  const negative: string[] = [];
  const positive: string[] = [];
  dependencyTypes.forEach((name) => {
    if (name.startsWith('!')) {
      negative.push(name.replace('!', ''));
    } else {
      positive.push(name);
    }
  });
  if (isNonEmptyArray(negative) && !negative.includes(instance.strategy.name)) return true;
  return isNonEmptyArray(positive) && positive.includes(instance.strategy.name);
}
