import { isNonEmptyString } from 'expect-more';
import type { VersionGroup } from '..';
import type { Instance } from '../../../get-package-json-files/package-json-file/instance';
import { getHighestVersion } from './get-highest-version';

export class InstanceGroup {
  hasMismatches: boolean;
  instances: Instance[];
  isBanned: boolean;
  isIgnored: boolean;
  isInvalid: boolean;
  isUnpinned: boolean;
  isWorkspace: boolean;
  name: string;
  uniques: string[];
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
    this.isBanned = isBanned;
    this.isIgnored = isIgnored;
    this.isInvalid = isInvalid;
    this.isUnpinned = isUnpinned;
    this.name = name;
    this.uniques = uniques;
    this.versionGroup = versionGroup;
    this.isWorkspace = versionGroup.input.workspace === true;
  }

  getExpectedVersion(): string | undefined {
    // remove this dependency
    if (this.isBanned) {
      return undefined;
    }
    if (this.isUnpinned) {
      return this.getPinnedVersion();
    }
    if (this.isWorkspace) {
      return this.getWorkspaceVersion() || getHighestVersion(this.uniques);
    }
    return getHighestVersion(this.uniques);
  }

  getPinnedVersion() {
    return this.versionGroup.pinVersion || '';
  }

  /**
   * If the dependency `name` is a package developed locally in this monorepo,
   * we should use its version as the source of truth.
   */
  getWorkspaceVersion() {
    return (
      this.instances.find((instance) => instance.dependencyType === 'workspace')
        ?.packageJsonFile.contents.version || ''
    );
  }
}
