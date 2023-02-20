import { O, pipe } from '@mobily/ts-belt';
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
  isBanned(): boolean {
    return (this.groupConfig as Banned).isBanned === true;
  }

  isIgnored(): boolean {
    return (this.groupConfig as Ignored).isIgnored === true;
  }

  hasSnappedToPackages(): boolean {
    return O.isSome(this.getSnappedToPackages());
  }

  getSnappedToPackages(): O.Option<string[]> {
    return O.fromPredicate(
      (this.groupConfig as SnappedTo).snapTo,
      isArrayOfStrings,
    );
  }

  getAllInstanceGroups(): InstanceGroup[] {
    return Object.entries(this.instancesByName).map(
      ([name, instances]) => new InstanceGroup(this, name, instances),
    );
  }

  getInvalidInstanceGroups(): InstanceGroup[] {
    return this.getAllInstanceGroups().filter((group) => group.isInvalid());
  }

  getPinnedVersion(): O.Option<string> {
    return O.fromPredicate(
      (this.groupConfig as Pinned).pinVersion,
      isNonEmptyString,
    );
  }

  /** Is `pinVersion` defined and this group does not match that version? */
  isUnpinned(): boolean {
    return pipe(
      this.getPinnedVersion(),
      O.map((pinVersion) =>
        this.instances.some((o) => o.version !== pinVersion),
      ),
      O.getWithDefault<boolean>(false),
    );
  }
}
