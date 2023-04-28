import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isEmptyArray } from 'tightrope/guard/is-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Context } from '../get-context';
import { NameAndVersionPropsStrategy } from '../strategy/name-and-version-props';
import { VersionsByNameStrategy } from '../strategy/versions-by-name';
import type { Strategy } from './get-custom-types';
import { getCustomTypes } from './get-custom-types';

export function getEnabledTypes({
  cli,
  rcFile,
}: Context['config']): Strategy.Any[] {
  const enabledTypes: Strategy.Any[] = [];
  const enabledTypeNames = (
    isNonEmptyString(cli.types)
      ? cli.types.split(',')
      : isArrayOfStrings(rcFile.dependencyTypes)
      ? rcFile.dependencyTypes
      : []
  ).filter(isNonEmptyString);
  // @TODO accept `dependencyTypes: ['**']`
  // @TODO support `dependencyTypes: ['!dev']`
  const useDefaults = isEmptyArray(enabledTypeNames);

  if (useDefaults || enabledTypeNames.includes('dev')) {
    enabledTypes.push(new VersionsByNameStrategy('dev', 'devDependencies'));
  }
  if (useDefaults || enabledTypeNames.includes('overrides')) {
    enabledTypes.push(new VersionsByNameStrategy('overrides', 'overrides'));
  }
  if (useDefaults || enabledTypeNames.includes('peer')) {
    enabledTypes.push(new VersionsByNameStrategy('peer', 'peerDependencies'));
  }
  if (useDefaults || enabledTypeNames.includes('pnpmOverrides')) {
    enabledTypes.push(
      new VersionsByNameStrategy('pnpmOverrides', 'pnpm.overrides'),
    );
  }
  if (useDefaults || enabledTypeNames.includes('prod')) {
    enabledTypes.push(new VersionsByNameStrategy('prod', 'dependencies'));
  }
  if (useDefaults || enabledTypeNames.includes('resolutions')) {
    enabledTypes.push(new VersionsByNameStrategy('resolutions', 'resolutions'));
  }
  if (useDefaults || enabledTypeNames.includes('workspace')) {
    enabledTypes.push(
      new NameAndVersionPropsStrategy('workspace', 'version', 'name'),
    );
  }

  getCustomTypes({ cli, rcFile }).forEach((customType) => {
    if (useDefaults || enabledTypeNames.includes(customType.name)) {
      enabledTypes.push(customType);
    }
  });

  return enabledTypes;
}
