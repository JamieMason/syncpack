import { Effect, Option, pipe } from 'effect';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import type { PackageJsonFile } from '../../get-package-json-files/package-json-file.js';
import { get } from '../../lib/get.js';

const getOptionOfNonEmptyString = Option.liftPredicate(isNonEmptyString);

export function getNonEmptyStringProp(
  propPath: string,
  file: PackageJsonFile,
): Effect.Effect<never, unknown, string> {
  return pipe(
    get(file.jsonFile.contents, ...propPath.split('.')),
    Effect.flatMap((value) => getOptionOfNonEmptyString(value)),
    Effect.tapError(() =>
      Effect.logDebug(`<${file.jsonFile.shortPath}>.${propPath} is not a non-empty string`),
    ),
  );
}
