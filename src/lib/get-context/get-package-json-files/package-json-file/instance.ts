import { isNonEmptyArray } from 'expect-more';
import minimatch from 'minimatch';
import type { PackageJsonFile } from '.';
import { setSemverRange } from '../../../set-semver-range';
import type { DependencyType, ValidRange } from '../../get-config/config';
import type { SemverGroup } from '../../get-groups/semver-group';
import type { VersionGroup } from '../../get-groups/version-group';

export class Instance {
  /** where this dependency is installed */
  dependencyType: DependencyType;
  /** the name of this dependency */
  name: string;
  /** The package this dependency is installed in this specific time */
  packageJsonFile: PackageJsonFile;
  /** The .name property of the package.json file of this instance */
  pkgName: string;
  /** the version of this dependency */
  version: string;

  constructor(
    dependencyType: DependencyType,
    name: string,
    packageJsonFile: PackageJsonFile,
    version: string,
  ) {
    this.dependencyType = dependencyType;
    this.name = name;
    this.packageJsonFile = packageJsonFile;
    this.pkgName = packageJsonFile.contents.name || 'PACKAGE_JSON_HAS_NO_NAME';
    this.version = version;
  }

  hasRange(range: ValidRange): boolean {
    return this.version === setSemverRange(range, this.version);
  }

  setRange(range: ValidRange): void {
    this.setVersion(setSemverRange(range, this.version));
  }

  /**
   * In the case of banned dependencies, their version is set to `undefined`,
   * which causes them to be removed by `JSON.stringify`.
   */
  setVersion(version: string | undefined): void {
    const root: any = this.packageJsonFile.contents;
    if (this.dependencyType === 'pnpmOverrides') {
      root.pnpm.overrides[this.name] = version;
    } else if (this.dependencyType !== 'workspace') {
      root[this.dependencyType][this.name] = version;
    }
  }

  matchesGroup(group: SemverGroup | VersionGroup): boolean {
    return (
      group.packages.some((pattern) => minimatch(this.pkgName, pattern)) &&
      group.dependencies.some((pattern) => minimatch(this.name, pattern)) &&
      (!isNonEmptyArray(group.dependencyTypes) ||
        group.dependencyTypes.includes(this.dependencyType))
    );
  }
}
