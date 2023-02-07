import { isNonEmptyString } from 'expect-more';
import type { VersionGroup } from '..';
import type { Instance } from '../../../get-package-json-files/package-json-file/instance';
import { getHighestVersion } from './get-highest-version';

/** Every `Instance` of eg `"lodash"` for a given `VersionGroup` */
export class InstanceGroup {
  /** 1+ `Instance` has a version which does not follow the rules */
  hasMismatches: boolean;
  /** Every package/dependencyType location where this dependency was found */
  instances: Instance[];
  /** Syncpack must report or fix this groups mismatches */
  isInvalid: boolean;
  /** 1+ `Instance` has a version not matching `VersionGroup.pinVersion` */
  isUnpinned: boolean;
  /** @example `"lodash"` */
  name: string;
  /** All `Instance` versions, with duplicates removed */
  uniques: string[];
  /** The `VersionGroup` which this `InstanceGroup` belongs to */
  versionGroup: VersionGroup;

  constructor(versionGroup: VersionGroup, name: string, instances: Instance[]) {
    const pinnedVersion = versionGroup.pinVersion;
    const isBanned = versionGroup.isBanned === true;
    const isIgnored = versionGroup.isIgnored === true;
    const hasPinnedVersion = isNonEmptyString(pinnedVersion);
    const versions = instances.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const [version] = uniques;
    const isUnpinned = hasPinnedVersion && version !== pinnedVersion;
    const hasMismatches = isBanned || isUnpinned || uniques.length > 1;
    const isInvalid = !isIgnored && hasMismatches;

    this.hasMismatches = hasMismatches;
    this.instances = instances;
    this.isInvalid = isInvalid;
    this.isUnpinned = isUnpinned;
    this.name = name;
    this.uniques = uniques;
    this.versionGroup = versionGroup;
  }

  getExpectedVersion(): string | undefined {
    // remove this dependency
    if (this.versionGroup.isBanned) {
      return undefined;
    }
    if (this.isUnpinned) {
      return this.getPinnedVersion();
    }
    if (this.versionGroup.input.workspace) {
      return this.getWorkspaceVersion() || getHighestVersion(this.uniques);
    }
    return getHighestVersion(this.uniques);
  }

  getPinnedVersion() {
    return this.versionGroup.pinVersion || '';
  }

  /**
   * If this dependency is a package developed locally, we should use its
   * version as the source of truth.
   */
  getWorkspaceVersion() {
    return this.getWorkspaceInstance()?.packageJsonFile.contents.version || '';
  }

  /**
   * Find instance of this dependency which is a package developed locally in
   * this monorepo.
   */
  getWorkspaceInstance(): Instance | undefined {
    return this.instances.find(
      (instance) => instance.dependencyType === 'workspace',
    );
  }
}
