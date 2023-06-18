import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { Env } from '../env/create-env';
import type { GlobError, ReadFileError } from '../env/tags';
import type { Ctx } from '../get-context';
import type { NoSourcesFoundError } from './get-file-paths';
import { getFilePaths } from './get-file-paths';
import type { JsonParseError } from './get-patterns/read-json-safe';
import { readJsonSafe } from './get-patterns/read-json-safe';
import type { PackageJson } from './package-json-file';
import { PackageJsonFile } from './package-json-file';

/** Create an API for every package.json file needed. */
export function getPackageJsonFiles(
  config: Ctx['config'],
): Effect.Effect<
  Env,
  NoSourcesFoundError | GlobError | ReadFileError | JsonParseError,
  PackageJsonFile[]
> {
  return pipe(
    getFilePaths(config),
    Effect.flatMap((paths) => Effect.all(paths.map(readJsonSafe<PackageJson>))),
    Effect.map((files) => files.map((file) => new PackageJsonFile(file, config))),
  );
}
