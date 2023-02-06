import { isNonEmptyArray } from 'expect-more';
import minimatch from 'minimatch';
import type { PackageJsonFile } from '.';
import type { Config, DependencyType } from '../../get-config/config';

export class Instance {
  /** where this dependency is installed */
  dependencyType: DependencyType;
  /** if dependencyType is "customDependencies" his associated dependencyCustomPath */
  dependencyCustomPath?: string;
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
    dependencyCustomPath?: string,
  ) {
    this.dependencyType = dependencyType;
    this.dependencyCustomPath = dependencyCustomPath;
    this.name = name;
    this.packageJsonFile = packageJsonFile;
    this.pkgName = packageJsonFile.contents.name || 'PACKAGE_JSON_HAS_NO_NAME';
    this.version = version;
  }

  matchesGroup(group: Config.SemverGroup.Any | Config.VersionGroup.Any) {
    return (
      group.packages.some((pattern) => minimatch(this.pkgName, pattern)) &&
      group.dependencies.some((pattern) => minimatch(this.name, pattern)) &&
      (!isNonEmptyArray(group.dependencyTypes) ||
        group.dependencyTypes.includes(this.dependencyType))
    );
  }
}
