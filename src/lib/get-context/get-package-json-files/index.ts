import * as E from 'fp-ts/lib/Either';
import { flow, identity, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import type { Disk } from '../../disk';
import type { InternalConfig } from '../get-config/internal-config';
import { getFilePaths } from './get-file-paths';
import { readJsonSafe } from './get-patterns/read-json-safe';
import type { PackageJson } from './package-json-file';
import { PackageJsonFile } from './package-json-file';

/** Create an API for every package.json file needed. */
export function getPackageJsonFiles(
  disk: Disk,
  program: InternalConfig,
): PackageJsonFile[] {
  const useEmpty = () => [];
  return pipe(
    getFilePaths(disk, program),
    E.chain(
      flow(
        O.getOrElse(useEmpty as () => string[]),
        E.traverseArray(readJsonSafe<PackageJson>(disk)),
        E.map((jsonFiles) =>
          jsonFiles.map(
            (jsonFile) => new PackageJsonFile(jsonFile, program, disk),
          ),
        ),
      ),
    ),
    E.fold(useEmpty as () => PackageJsonFile[], identity),
  );
}
