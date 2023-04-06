import { uniq } from 'tightrope/array/uniq';
import { get } from 'tightrope/fn/get';
import { identity } from 'tightrope/fn/identity';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { filter as filterO } from 'tightrope/option/filter';
import { map as mapO } from 'tightrope/option/map';
import { okOr } from 'tightrope/option/ok-or';
import type { Result } from 'tightrope/result';
import { Err, Ok } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter as filterR } from 'tightrope/result/filter';
import { isOk } from 'tightrope/result/is-ok';
import { map as mapR } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import { unwrap } from 'tightrope/result/unwrap';
import type { VersionGroup } from '..';
import { BaseError } from '../../../../lib/error';
import { isSemver } from '../../../../lib/is-semver';
import { printStrings } from '../../../../lib/print-strings';
import type { Syncpack } from '../../../../types';
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
    return uniq(this.instances.map((obj) => obj.version)).sort();
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

  getExpectedVersion(): Result<string | Delete> {
    const versionGroup = this.versionGroup;
    if (versionGroup.isBanned()) return new Ok(DELETE);
    if (versionGroup.isUnpinned())
      return pipe(
        versionGroup.getPinnedVersion(),
        okOr(
          new BaseError(
            `${this.name} is in a versionGroup with pinVersion configuration, but the pinVersion value is not valid`,
          ),
        ),
      );
    if (versionGroup.hasSnappedToPackages() && unwrap(this.isUnsnapped()))
      return this.getSnappedVersion();
    if (this.hasWorkspaceInstance()) return this.getWorkspaceVersion();
    if (this.hasUnsupportedVersion()) {
      return new Err(
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
  getHighestVersion(): Result<string> {
    return getHighestVersion(this.getUniqueVersions());
  }

  /** If all versions are valid semver, return the lowest one */
  getLowestVersion(): Result<string> {
    return getLowestVersion(this.getUniqueVersions());
  }

  /** Get the first version matched by the `snapTo` packages */
  getSnappedVersion(): Result<string> {
    return pipe(
      this.versionGroup.getSnappedToPackages(),
      mapO(
        (pkgNames) =>
          this.instances
            .filter(({ pkgName }) => pkgNames.includes(pkgName))
            .map(({ version }) => version)
            .find(Boolean) || '',
      ),
      filterO<string>(isNonEmptyString),
      okOr(
        new BaseError(
          `${this.name} is in a versionGroup with snapTo configuration, but ${this.name} was not found in those packages`,
        ),
      ),
    );
  }

  /** Is `snapTo` defined and this group does not match that version? */
  isUnsnapped(): Result<boolean> {
    return this.versionGroup.hasSnappedToPackages()
      ? pipe(
          this.getSnappedVersion(),
          mapR((nextVersion) =>
            this.instances.some(({ version }) => version !== nextVersion),
          ),
        )
      : new Ok(false);
  }

  /** Get version of dependency which is developed in this monorepo */
  getWorkspaceVersion(): Result<string> {
    return pipe(
      this.getWorkspaceInstance(),
      andThen((instance) =>
        get(instance, 'packageJsonFile', 'contents', 'version'),
      ),
      filterR(isNonEmptyString, ''),
      identity as () => Result<string>,
      mapErr(
        () =>
          new BaseError(
            `Expected to find a package.json file developed in this monorepo with a "name" property of "${this.name}" and a valid "version" property`,
          ),
      ),
    );
  }

  /** Find instance of this dependency which is developed in this monorepo */
  getWorkspaceInstance(): Result<Instance> {
    const instance = this.instances.find(
      ({ pathDef }) => pathDef.name === 'workspace',
    );
    return instance
      ? new Ok(instance)
      : new Err(new BaseError('Workspace instance not found'));
  }

  /** Is an instance of this dependency developed in this monorepo? */
  hasWorkspaceInstance(): boolean {
    return isOk(this.getWorkspaceInstance());
  }
}
