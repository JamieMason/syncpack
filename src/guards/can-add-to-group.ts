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
    (!isNonEmptyArray(dependencyTypes) || dependencyTypes.includes(instance.strategy.name)) &&
    (!isNonEmptyArray(packages) ||
      packages.some((pattern) => minimatch(instance.pkgName, pattern))) &&
    (!isNonEmptyArray(dependencies) ||
      dependencies.some((pattern) => minimatch(instance.name, pattern)))
  );
}
