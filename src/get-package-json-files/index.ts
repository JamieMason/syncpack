import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { Env } from '../env/create-env';
import type { JsonParseError } from '../env/tags';
import { EnvTag, type GlobError, type ReadFileError } from '../env/tags';
import type { Ctx } from '../get-context';
import type { NoSourcesFoundError } from './get-file-paths';
import { getFilePaths } from './get-file-paths';
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
    Effect.Do,
    Effect.bind('env', () => EnvTag),
    Effect.bind('filePaths', () => getFilePaths(config)),
    Effect.bind('files', ({ env, filePaths }) =>
      Effect.all(filePaths.map(env.readJsonFileSync<PackageJson>)),
    ),
    Effect.map(({ files }) => files.map((file) => new PackageJsonFile(file, config))),
  );
}
