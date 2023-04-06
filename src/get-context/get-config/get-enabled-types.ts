import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Syncpack } from '../../types';

type Options = Pick<
  Partial<Syncpack.Config.Public>,
  'dependencyTypes' | 'types'
>;

export function getEnabledTypes(
  allTypes: Syncpack.PathDefinition[],
  { dependencyTypes, types }: Options,
): Syncpack.PathDefinition[] {
  const enabledNames = isNonEmptyString(types)
    ? types.split(',')
    : isArrayOfStrings(dependencyTypes)
    ? dependencyTypes
    : [];
  return isNonEmptyArray(enabledNames)
    ? allTypes.filter(({ name }) => enabledNames.includes(name))
    : allTypes;
}
