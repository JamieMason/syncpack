import { flow, pipe, R } from '@mobily/ts-belt';
import { $R } from '../$R';
import type { Syncpack } from '../../../types';
import type { Disk } from '../../disk';
import type { BaseError } from '../../error';
import { getFilePaths } from './get-file-paths';
import { readJsonSafe } from './get-patterns/read-json-safe';
import type { PackageJson } from './package-json-file';
import { PackageJsonFile } from './package-json-file';

/** Create an API for every package.json file needed. */
export function getPackageJsonFiles(
  disk: Disk,
  program: Syncpack.Config.Private,
): PackageJsonFile[] {
  return pipe(
    getFilePaths(disk, program),
    R.flatMap($R.onlyOk<string, PackageJsonFile>(resolvePackageJson(disk))),
    R.getWithDefault([] as PackageJsonFile[]),
  );

  function resolvePackageJson(
    disk: Disk,
  ): (filePath: string) => R.Result<PackageJsonFile, BaseError> {
    return flow(
      readJsonSafe<PackageJson>(disk),
      R.map((jsonFile) => new PackageJsonFile(jsonFile, program, disk)),
    );
  }
}
