import { pipe } from '@mobily/ts-belt';
import { isNonEmptyArray } from 'expect-more';
import minimatch from 'minimatch';
import type { PackageJsonFile } from '.';
import { $R } from '../../$R';
import { setSemverRange } from '../../../lib/set-semver-range';
import type { Syncpack } from '../../../types';
import {
  exhaustiveCheck,
  strategyByName,
} from '../../get-config/path-strategy';
import type { SemverGroup } from '../../get-groups/semver-group';
import type { VersionGroup } from '../../get-groups/version-group';

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

  hasRange(range: Syncpack.Config.SemverRange.Value): boolean {
    if (this.pathDef.name === 'workspace') {
      // version property of package.json must always be exact
      return this.version === setSemverRange('', this.version);
    }
    return this.version === setSemverRange(range, this.version);
  }

  setRange(range: Syncpack.Config.SemverRange.Value): void {
    this.setVersion(setSemverRange(range, this.version));
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

  matchesGroup(group: SemverGroup | VersionGroup): boolean {
    return (
      group.packages.some((pattern) => minimatch(this.pkgName, pattern)) &&
      group.dependencies.some((pattern) => minimatch(this.name, pattern)) &&
      (!isNonEmptyArray(group.dependencyTypes) ||
        group.dependencyTypes.includes(this.pathDef.name))
    );
  }
}
