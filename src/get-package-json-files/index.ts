import { flow } from 'tightrope/fn/flow';
import { pipe } from 'tightrope/fn/pipe';
import type { Result } from 'tightrope/result';
import { Ok } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { map } from 'tightrope/result/map';
import { or } from 'tightrope/result/or';
import type { Context } from '../get-context';
import { $R } from '../lib/$R';
import type { Disk } from '../lib/disk';
import { getFilePaths } from './get-file-paths';
import { readJsonSafe } from './get-patterns/read-json-safe';
import type { PackageJson } from './package-json-file';
import { PackageJsonFile } from './package-json-file';

/** Create an API for every package.json file needed. */
export function getPackageJsonFiles(
  disk: Disk,
  config: Context['config'],
): Result<PackageJsonFile[]> {
  return pipe(
    getFilePaths(disk, config),
    andThen(
      $R.onlyOk(
        flow(
          readJsonSafe<PackageJson>(disk),
          map((jsonFile) => new PackageJsonFile(jsonFile, config, disk)),
          $R.tapErrVerbose,
        ),
      ),
    ),
    or(new Ok([])),
  );
}
