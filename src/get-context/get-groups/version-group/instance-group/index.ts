import { R } from '@mobily/ts-belt';
import type { VersionGroup } from '..';
import { BaseError } from '../../../../lib/error';
import { isSemver } from '../../../../lib/is-semver';
import { printStrings } from '../../../../lib/print-strings';
import type { Instance } from '../../../get-package-json-files/package-json-file/instance';
import { getHighestVersion } from './get-highest-version';

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

  hasUnsupportedVersion() {
    return this.instances.some((obj) => !isSemver(obj.version));
  }

  getUniqueVersions() {
    return Array.from(new Set(this.instances.map((obj) => obj.version))).sort();
  }

  hasMismatchingVersions() {
    return this.getUniqueVersions().length > 1;
  }

  isInvalid() {
    return this.versionGroup.isIgnored()
      ? false
      : this.versionGroup.isBanned() ||
          this.versionGroup.isUnpinned() ||
          this.hasMismatchingVersions();
  }

  getExpectedVersion(): string | undefined {
    const versionGroup = this.versionGroup;
    const REMOVE_DEPENDENCY = undefined;
    if (versionGroup.isBanned()) return REMOVE_DEPENDENCY;
    if (versionGroup.isUnpinned()) return versionGroup.getPinnedVersion();
    if (this.hasWorkspaceInstance()) return this.getWorkspaceVersion();
    if (this.hasUnsupportedVersion()) {
      throw new BaseError(
        `${this.name} contains unsupported versions: ${printStrings(
          this.getUniqueVersions(),
        )}`,
      );
    }
    return R.getExn(this.getHighestVersion());
  }

  getHighestVersion() {
    return getHighestVersion(this.getUniqueVersions());
  }

  isUnpinned() {
    return (
      this.versionGroup.hasPinnedVersion() &&
      this.instances.some(
        ({ version }) => version !== this.versionGroup.getPinnedVersion(),
      )
    );
  }

  /** Get version of dependency which is developed in this monorepo */
  getWorkspaceVersion() {
    if (this.hasWorkspaceInstance()) {
      return this.getWorkspaceInstance()?.packageJsonFile.contents.version;
    }
    throw new BaseError('getWorkspaceVersion invoked when there is none');
  }

  /** Find instance of this dependency which is developed in this monorepo */
  getWorkspaceInstance(): Instance | undefined {
    return this.instances.find(({ pathDef }) => pathDef.name === 'workspace');
  }

  hasWorkspaceInstance(): boolean {
    return this.getWorkspaceInstance() !== undefined;
  }
}
