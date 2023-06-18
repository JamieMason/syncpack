import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import { CWD } from '../../constants';
import type { Env } from '../../env/create-env';
import type { PackageJson } from '../package-json-file';
import { readJsonSafe } from './read-json-safe';

export function getYarnPatterns(): Effect.Effect<Env, never, O.Option<string[]>> {
  return pipe(
    readJsonSafe<PackageJson>(join(CWD, 'package.json')),
    Effect.map((file) =>
      isNonEmptyObject(file?.contents?.workspaces) &&
      isArrayOfStrings(file.contents.workspaces?.packages)
        ? O.some(file.contents.workspaces.packages)
        : isArrayOfStrings(file?.contents?.workspaces)
        ? O.some(file.contents.workspaces)
        : O.none(),
    ),
    Effect.catchTags({
      ReadFileError: () => Effect.succeed(O.none()),
      JsonParseError: () => Effect.succeed(O.none()),
    }),
  );
}
