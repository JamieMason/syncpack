import { sortByName } from '../lib/sort-by-name';
import type { PackageJsonFile } from './get-package-json-files/package-json-file';
import type { Instance } from './get-package-json-files/package-json-file/instance';

export function getAllInstances(
  packageJsonFiles: PackageJsonFile[],
): Instance[] {
  return packageJsonFiles.flatMap((pkg) => pkg.getInstances()).sort(sortByName);
}
