import { isNonEmptyArray } from 'expect-more/dist/is-non-empty-array';
import minimatch from 'minimatch';
import type { Syncpack } from '../../types';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Instance } from '../get-package-json-files/package-json-file/instance';

export class BaseGroup<
  T extends Syncpack.Config.SemverGroup.Any | Syncpack.Config.VersionGroup.Any,
> {
  /** Full config for this run of syncpack */
  syncpackConfig: Syncpack.Config.Private;
  /** The original config which created this `SemverGroup` */
  groupConfig: T;
  /** Instances which belong to this group */
  instances: Instance[];
  /** Instances which belong to this group, keyed by their .name property */
  instancesByName: Record<string, Instance[]>;
  /** Is this the catch-all group, not defined by the user? */
  isDefault: boolean;
  /** All package.json files */
  packageJsonFiles: PackageJsonFile[];

  constructor(
    config: Syncpack.Config.Private,
    group: T,
    packageJsonFiles: PackageJsonFile[],
  ) {
    this.groupConfig = group;
    this.instances = [];
    this.instancesByName = {};
    this.isDefault =
      group === config.defaultSemverGroup ||
      group === config.defaultVersionGroup;
    this.packageJsonFiles = packageJsonFiles;
    this.syncpackConfig = config;
  }

  /** Can this instance be added to this group? */
  canAdd(instance: Instance): boolean {
    const { dependencies, dependencyTypes, packages } = this.groupConfig;
    return (
      (!isNonEmptyArray(dependencyTypes) ||
        dependencyTypes.includes(instance.pathDef.name)) &&
      (!isNonEmptyArray(packages) ||
        packages.some((pattern) => minimatch(instance.pkgName, pattern))) &&
      (!isNonEmptyArray(dependencies) ||
        dependencies.some((pattern) => minimatch(instance.name, pattern)))
    );
  }

  /** Add this instance to this group */
  add(instance: Instance): void {
    if (!this.instancesByName[instance.name]) {
      this.instancesByName[instance.name] = [];
    }
    this.instancesByName[instance.name]?.push(instance);
    this.instances.push(instance);
  }
}
