import { isArrayOfStrings } from 'expect-more';
import { isNonEmptyString } from 'expect-more/dist/is-non-empty-string';
import type { Syncpack } from '../../../types';
import { BaseGroup } from '../base-group';
import { InstanceGroup } from './instance-group';

type Banned = Syncpack.Config.VersionGroup.Banned;
type Ignored = Syncpack.Config.VersionGroup.Ignored;
type Pinned = Syncpack.Config.VersionGroup.Pinned;
type SnappedTo = Syncpack.Config.VersionGroup.SnappedTo;

export class VersionGroup extends BaseGroup<Syncpack.Config.VersionGroup.Any> {
  getAllInstanceGroups(): InstanceGroup[] {
    return Object.entries(this.instancesByName).map(
      ([name, instances]) => new InstanceGroup(this, name, instances),
    );
  }

  getInvalidInstanceGroups(): InstanceGroup[] {
    return this.getAllInstanceGroups().filter((group) => group.isInvalid());
  }

  isBanned(): boolean {
    return (this.groupConfig as Banned).isBanned === true;
  }

  isIgnored(): boolean {
    return (this.groupConfig as Ignored).isIgnored === true;
  }

  hasSnappedToPackages(): boolean {
    return isArrayOfStrings((this.groupConfig as SnappedTo).snapTo);
  }

  getSnappedToPackages(): string[] {
    return (this.groupConfig as SnappedTo).snapTo;
  }

  getPinnedVersion(): string {
    return (this.groupConfig as Pinned).pinVersion;
  }

  isUnpinned(): boolean {
    const { pinVersion } = this.groupConfig as Pinned;
    return (
      isNonEmptyString(pinVersion) &&
      this.instances.some(({ version }) => version !== pinVersion)
    );
  }
}
