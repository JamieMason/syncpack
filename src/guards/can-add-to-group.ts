import { minimatch } from 'minimatch';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array.js';
import type { Ctx } from '../get-context/index.js';
import type { Instance } from '../get-instances/instance.js';
import type { SemverGroup } from '../semver-group/index.js';
import type { VersionGroup } from '../version-group/index.js';

export function canAddToGroup(
  packageJsonFilesByName: Ctx['packageJsonFilesByName'],
  group: SemverGroup.Any | VersionGroup.Any,
  instance: Instance,
): boolean {
  const { dependencies, dependencyTypes, packages, specifierTypes } = group.config;
  return (
    group.canAdd(instance) &&
    matchesDependencyTypes(dependencyTypes, instance) &&
    matchesPackages(packages, instance) &&
    matchesDependencies(packageJsonFilesByName, group, dependencies, instance) &&
    matchesSpecifierTypes(specifierTypes, instance)
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

function matchesDependencyTypes(dependencyTypes: unknown, instance: Instance): boolean {
  return matchesKnownList(dependencyTypes, instance.strategy.name);
}

function matchesSpecifierTypes(specifierTypes: unknown, instance: Instance): boolean {
  return matchesKnownList(specifierTypes, instance.rawSpecifier.name);
}

function matchesKnownList(values: unknown, value: string): boolean {
  // matches if not defined
  if (!isNonEmptyArray(values)) return true;
  if (values.join('') === '**') return true;
  const negative: string[] = [];
  const positive: string[] = [];
  values.forEach((name) => {
    if (name.startsWith('!')) {
      negative.push(name.replace('!', ''));
    } else {
      positive.push(name);
    }
  });
  if (isNonEmptyArray(negative) && !negative.includes(value)) return true;
  return isNonEmptyArray(positive) && positive.includes(value);
}
