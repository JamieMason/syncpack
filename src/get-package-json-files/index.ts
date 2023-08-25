import { Effect, pipe } from 'effect';
import type { Ctx } from '../get-context';
import type { Io } from '../io';
import type { GlobError } from '../io/glob-sync';
import type { ReadFileError } from '../io/read-file-sync';
import type { JsonParseError } from '../io/read-json-file-sync';
import { readJsonFileSync } from '../io/read-json-file-sync';
import type { NoSourcesFoundError } from './get-file-paths';
import { getFilePaths } from './get-file-paths';
import type { PackageJson } from './package-json-file';
import { PackageJsonFile } from './package-json-file';

/** Create an API for every package.json file needed. */
export function getPackageJsonFiles(
  io: Io,
  config: Ctx['config'],
): Effect.Effect<
  never,
  NoSourcesFoundError | GlobError | ReadFileError | JsonParseError,
  PackageJsonFile[]
> {
  return pipe(
    getFilePaths(io, config),
    Effect.flatMap((filePaths) =>
      Effect.all(filePaths.map((filePath) => readJsonFileSync<PackageJson>(io, filePath))),
    ),
    Effect.map((files) => files.map((file) => new PackageJsonFile(file, config))),
    Effect.tap((files) => Effect.logDebug(`${files.length} package.json files found`)),
  );
}
