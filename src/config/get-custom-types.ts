import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import type { Context } from '../get-context';
import { NameAndVersionPropsStrategy } from '../strategy/name-and-version-props';
import { NamedVersionStringStrategy } from '../strategy/named-version-string';
import { UnnamedVersionStringStrategy } from '../strategy/unnamed-version-string';
import { VersionsByNameStrategy } from '../strategy/versions-by-name';

export namespace Strategy {
  export type Any =
    | NameAndVersionPropsStrategy
    | NamedVersionStringStrategy
    | UnnamedVersionStringStrategy
    | VersionsByNameStrategy;
}

export function getCustomTypes({ rcFile }: Context['config']): Strategy.Any[] {
  if (!isNonEmptyObject(rcFile.customTypes)) return [];
  const ERR_OBJ = new Error('Invalid customType');
  const ERR_NAME = new Error('Invalid customType name');
  const ERR_PATH = new Error('Invalid customType.path');
  const ERR_NAME_PATH = new Error('Invalid customType namePath');
  const ERR_STRATEGY = new Error('Invalid customType.strategy');

  return Object.entries(rcFile.customTypes).map(([name, config]) => {
    if (!isObject(config)) throw ERR_OBJ;
    if (!isNonEmptyString(name)) throw ERR_NAME;
    if (!isNonEmptyString(config.path)) throw ERR_PATH;

    const path = config.path;
    const strategy = config.strategy;

    switch (strategy) {
      case 'name~version': {
        const namePath = config.namePath;
        if (!isNonEmptyString(namePath)) throw ERR_NAME_PATH;
        return new NameAndVersionPropsStrategy(name, path, namePath);
      }
      case 'name@version': {
        return new NamedVersionStringStrategy(name, path);
      }
      case 'version': {
        return new UnnamedVersionStringStrategy(name, path);
      }
      case 'versionsByName': {
        return new VersionsByNameStrategy(name, path);
      }
      default:
        throw ERR_STRATEGY;
    }
  });
}
