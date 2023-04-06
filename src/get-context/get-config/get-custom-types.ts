import { isObject } from 'tightrope/guard/is-object';
import type { Syncpack } from '../../types';

type Options = Pick<Partial<Syncpack.Config.Public>, 'customTypes'>;

export function getCustomTypes({
  customTypes,
}: Options): Syncpack.PathDefinition[] {
  return customTypes
    ? Object.keys(customTypes)
        .map((name) => {
          const pathDef = customTypes[name];
          if (pathDef) return { ...pathDef, name };
        })
        .filter(isObject<Syncpack.PathDefinition>)
    : [];
}
