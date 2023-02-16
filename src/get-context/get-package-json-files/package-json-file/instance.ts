import { pipe } from '@mobily/ts-belt';
import type { PackageJsonFile } from '.';
import { $R } from '../../$R';
import type { Syncpack } from '../../../types';
import {
  exhaustiveCheck,
  strategyByName,
} from '../../get-config/path-strategy';

type Entry = [string, string | undefined];

export class Instance {
  /** the name of this dependency */
  name: string;
  /** The package this dependency is installed in this specific time */
  packageJsonFile: PackageJsonFile;
  /** where this dependency is installed */
  pathDef: Syncpack.PathDefinition;
  /** The .name property of the package.json file of this instance */
  pkgName: string;
  /** the version of this dependency */
  version: string;

  constructor(
    pathDef: Syncpack.PathDefinition,
    name: string,
    packageJsonFile: PackageJsonFile,
    version: string,
  ) {
    this.pathDef = pathDef;
    this.name = name;
    this.packageJsonFile = packageJsonFile;
    this.pkgName = packageJsonFile.contents.name || 'PACKAGE_JSON_HAS_NO_NAME';
    this.version = version;
  }

  /** Is this instance the package.json file of this package developed in this repo? */
  isWorkspace() {
    return this.pathDef.name === 'workspace';
  }

  /**
   * In the case of banned dependencies, their version is set to `undefined`,
   * which causes them to be removed by `JSON.stringify`.
   */
  setVersion(version: string | undefined): void {
    const strategyName = this.pathDef.strategy;
    const entry: Entry = [this.name, version];
    const file = this.packageJsonFile;
    switch (strategyName) {
      case 'name@version':
        pipe(
          strategyByName[strategyName].write(file, this.pathDef, entry),
          $R.tapErrVerbose,
        );
        break;
      case 'name~version':
        pipe(
          strategyByName[strategyName].write(file, this.pathDef, entry),
          $R.tapErrVerbose,
        );
        break;
      case 'version':
        pipe(
          strategyByName[strategyName].write(file, this.pathDef, entry),
          $R.tapErrVerbose,
        );
        break;
      case 'versionsByName':
        pipe(
          strategyByName[strategyName].write(file, this.pathDef, entry),
          $R.tapErrVerbose,
        );
        break;
      default:
        return exhaustiveCheck(strategyName);
    }
  }
}
