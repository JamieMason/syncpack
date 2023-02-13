import {
  isArrayOfStrings,
  isNonEmptyArray,
  isNonEmptyString,
} from 'expect-more';
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
