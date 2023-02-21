import { O, pipe, R } from '@mobily/ts-belt';
import { isNonEmptyString } from 'expect-more';
import type { VersionGroup } from '..';
import { BaseError } from '../../../../lib/error';
import { isSemver } from '../../../../lib/is-semver';
import { printStrings } from '../../../../lib/print-strings';
import type { Syncpack } from '../../../../types';
import { props } from '../../../get-package-json-files/get-patterns/props';
import type { Instance } from '../../../get-package-json-files/package-json-file/instance';
import { getHighestVersion } from './get-highest-version';
import { getLowestVersion } from './get-lowest-version';

type Standard = Syncpack.Config.VersionGroup.Standard;

export const DELETE = Symbol('DELETE');
export type Delete = typeof DELETE;

/** Every `Instance` of eg `"lodash"` for a given `VersionGroup` */
export class InstanceGroup {
  /** Every package/pathName location where this dependency was found */
  instances: Instance[];
  /** @example `"lodash"` */
  name: string;
  /** The `VersionGroup` which this `InstanceGroup` belongs to */
  versionGroup: VersionGroup;

  constructor(versionGroup: VersionGroup, name: string, instances: Instance[]) {
    this.instances = instances;
    this.name = name;
    this.versionGroup = versionGroup;
  }

  hasUnsupportedVersion(): boolean {
    return this.instances.some((obj) => !isSemver(obj.version));
  }

  getUniqueVersions(): string[] {
    return Array.from(new Set(this.instances.map((obj) => obj.version))).sort();
  }

  hasMismatchingVersions(): boolean {
    return this.getUniqueVersions().length > 1;
  }

  isInvalid(): boolean {
    return this.versionGroup.isIgnored()
      ? false
      : this.versionGroup.isBanned() ||
          this.versionGroup.isUnpinned() ||
          this.hasMismatchingVersions();
  }

  getExpectedVersion(): R.Result<string | Delete, BaseError> {
    const versionGroup = this.versionGroup;
    if (versionGroup.isBanned()) return R.Ok(DELETE);
    if (versionGroup.isUnpinned())
      return pipe(
        versionGroup.getPinnedVersion(),
        O.toResult(
          new BaseError(
            `${this.name} is in a versionGroup with pinVersion configuration, but the pinVersion value is not valid`,
          ),
        ),
      );
    if (versionGroup.hasSnappedToPackages() && R.getExn(this.isUnsnapped()))
      return this.getSnappedVersion();
    if (this.hasWorkspaceInstance()) return this.getWorkspaceVersion();
    if (this.hasUnsupportedVersion()) {
      return R.Error(
        new BaseError(
          `${this.name} contains unsupported versions: ${printStrings(
            this.getUniqueVersions(),
          )}`,
        ),
      );
    }
    return (versionGroup.groupConfig as Standard).preferVersion ===
      'lowestSemver'
      ? this.getLowestVersion()
      : this.getHighestVersion();
  }

  /** If all versions are valid semver, return the newest one */
  getHighestVersion(): R.Result<string, BaseError> {
    return getHighestVersion(this.getUniqueVersions());
  }

  /** If all versions are valid semver, return the lowest one */
  getLowestVersion(): R.Result<string, BaseError> {
    return getLowestVersion(this.getUniqueVersions());
  }

  /** Get the first version matched by the `snapTo` packages */
  getSnappedVersion(): R.Result<string, BaseError> {
    return pipe(
      this.versionGroup.getSnappedToPackages(),
      O.flatMap((pkgNames) =>
        O.fromFalsy(
          this.instances
            .filter(({ pkgName }) => pkgNames.includes(pkgName))
            .map(({ version }) => version)
            .find(Boolean),
        ),
      ),
      O.filter<string>(isNonEmptyString),
      O.toResult(
        new BaseError(
          `${this.name} is in a versionGroup with snapTo configuration, but ${this.name} was not found in those packages`,
        ),
      ),
    );
  }

  /** Is `snapTo` defined and this group does not match that version? */
  isUnsnapped(): R.Result<boolean, BaseError> {
    return this.versionGroup.hasSnappedToPackages()
      ? pipe(
          this.getSnappedVersion(),
          R.map((nextVersion) =>
            this.instances.some(({ version }) => version !== nextVersion),
          ),
        )
      : R.Ok(false);
  }

  /** Get version of dependency which is developed in this monorepo */
  getWorkspaceVersion(): R.Result<string, BaseError> {
    return pipe(
      this.getWorkspaceInstance(),
      props('packageJsonFile.contents.version', isNonEmptyString),
      O.toResult(
        new BaseError(
          `Expected to find a package.json file developed in this monorepo with a "name" property of "${this.name}" and a valid "version" property`,
        ),
      ),
    );
  }

  /** Find instance of this dependency which is developed in this monorepo */
  getWorkspaceInstance(): O.Option<Instance> {
    return O.fromFalsy(
      this.instances.find(({ pathDef }) => pathDef.name === 'workspace'),
    );
  }

  /** Is an instance of this dependency developed in this monorepo? */
  hasWorkspaceInstance(): boolean {
    return pipe(this.getWorkspaceInstance(), O.toUndefined) !== undefined;
  }
}
