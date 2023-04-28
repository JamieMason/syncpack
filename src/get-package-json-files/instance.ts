import { pipe } from 'tightrope/fn/pipe';
import type { Strategy } from '../config/get-custom-types';
import type { Delete } from '../get-version-groups/lib/delete';
import { $R } from '../lib/$R';
import type { PackageJsonFile } from './package-json-file';

export class Instance {
  /** the name of this dependency */
  name: string;
  /** The package this dependency is installed in this specific time */
  packageJsonFile: PackageJsonFile;
  /** locates where in the file this dependency is installed */
  strategy: Strategy.Any;
  /** The .name property of the package.json file of this instance */
  pkgName: string;
  /** the version of this dependency */
  version: string;

  constructor(
    strategy: Strategy.Any,
    name: string,
    packageJsonFile: PackageJsonFile,
    version: string,
  ) {
    this.strategy = strategy;
    this.name = name;
    this.packageJsonFile = packageJsonFile;
    this.pkgName = packageJsonFile.contents.name || 'PACKAGE_JSON_HAS_NO_NAME';
    this.version = version;
  }

  /**
   * In the case of banned dependencies, their version is set to `undefined`,
   * which causes them to be removed by `JSON.stringify`.
   */
  setVersion(version: string | Delete): void {
    const file = this.packageJsonFile;
    pipe(this.strategy.write(file, [this.name, version]), $R.tapErrVerbose);
  }
}
