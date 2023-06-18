import { minimatch } from 'minimatch';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import type { Instance } from '../get-package-json-files/instance';
import type { AnySemverGroup } from '../get-semver-groups';
import type { AnyVersionGroup } from '../get-version-groups';

export function canAddToGroup(
  group: AnySemverGroup | AnyVersionGroup,
  instance: Instance,
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
