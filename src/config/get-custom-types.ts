import { Data, Effect } from 'effect';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object.js';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import { isObject } from 'tightrope/guard/is-object.js';
import { DEFAULT_CONFIG } from '../constants.js';
import type { Ctx } from '../get-context/index.js';
import { NameAndVersionPropsStrategy } from '../strategy/name-and-version-props.js';
import { NamedVersionStringStrategy } from '../strategy/named-version-string.js';
import { UnnamedVersionStringStrategy } from '../strategy/unnamed-version-string.js';
import { VersionsByNameStrategy } from '../strategy/versions-by-name.js';

export namespace Strategy {
  export type Any =
    | NameAndVersionPropsStrategy
    | NamedVersionStringStrategy
    | UnnamedVersionStringStrategy
    | VersionsByNameStrategy;
}

export class InvalidCustomTypeError extends Data.TaggedClass('InvalidCustomTypeError')<{
  readonly config: unknown;
  readonly reason: string;
}> {}

export function getCustomTypes({
  rcFile,
}: Ctx['config']): Effect.Effect<never, InvalidCustomTypeError, Strategy.Any[]> {
  if (!isNonEmptyObject(rcFile.customTypes)) return Effect.succeed([]);

  return Effect.all(
    [...Object.entries(rcFile.customTypes), ...Object.entries(DEFAULT_CONFIG.customTypes)].map(
      ([name, config]) => {
        const ERR_OBJ = 'Invalid customType';
        const ERR_PATH = 'Invalid customType.path';
        const ERR_NAME_PATH = 'Invalid customType.namePath';
        const ERR_STRATEGY = 'Invalid customType.strategy';

        if (!isObject(config)) return createError(config, ERR_OBJ);
        if (!isNonEmptyString(config.path)) return createError(config, ERR_PATH);

        const path = config.path;
        const strategy = config.strategy;

        switch (strategy) {
          case 'name~version': {
            const namePath = config.namePath;
            if (!isNonEmptyString(namePath)) return createError(config, ERR_NAME_PATH);
            return Effect.succeed(new NameAndVersionPropsStrategy(name, path, namePath));
          }
          case 'name@version': {
            return Effect.succeed(new NamedVersionStringStrategy(name, path));
          }
          case 'version': {
            return Effect.succeed(new UnnamedVersionStringStrategy(name, path));
          }
          case 'versionsByName': {
            return Effect.succeed(new VersionsByNameStrategy(name, path));
          }
          default: {
            return createError(config, ERR_STRATEGY);
          }
        }
      },
    ),
  );
}

function createError(config: unknown, reason: string) {
  return Effect.fail(new InvalidCustomTypeError({ config, reason }));
}
