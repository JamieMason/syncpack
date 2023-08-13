import { minimatch } from 'minimatch';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import type { AnySemverGroup } from '../get-semver-groups';
import type { AnyVersionGroup } from '../get-version-groups';
import type { Instance } from '../instance';

export function canAddToGroup(
  group: AnySemverGroup | AnyVersionGroup,
  instance: Instance.Any,
): boolean {
  const { dependencies, dependencyTypes, packages } = group.config;
  return (
    group.canAdd(instance) &&
    matchesDependencyTypes(dependencyTypes, instance) &&
    matchesPackages(packages, instance) &&
    matchesDependencies(dependencies, instance)
  );
}

function matchesDependencies(dependencies: string[], instance: Instance.Any): boolean {
  // matches if not defined
  if (!isNonEmptyArray(dependencies)) return true;
  return dependencies.some((pattern) => minimatch(instance.name, pattern));
}

function matchesPackages(packages: string[], instance: Instance.Any) {
  // matches if not defined
  if (!isNonEmptyArray(packages)) return true;
  return packages.some((pattern) => minimatch(instance.pkgName, pattern));
}

function matchesDependencyTypes(dependencyTypes: string[] | undefined, instance: Instance.Any) {
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
