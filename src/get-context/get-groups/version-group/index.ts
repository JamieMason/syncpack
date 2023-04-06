import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Option } from 'tightrope/option';
import { fromGuard } from 'tightrope/option/from-guard';
import { isSome } from 'tightrope/option/is-some';
import { map as mapO } from 'tightrope/option/map';
import { unwrapOr } from 'tightrope/option/unwrap-or';
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
    return isSome(this.getSnappedToPackages());
  }

  getSnappedToPackages(): Option<string[]> {
    return fromGuard(isArrayOfStrings, (this.groupConfig as SnappedTo).snapTo);
  }

  getAllInstanceGroups(): InstanceGroup[] {
    return Object.entries(this.instancesByName).map(
      ([name, instances]) => new InstanceGroup(this, name, instances),
    );
  }

  getInvalidInstanceGroups(): InstanceGroup[] {
    return this.getAllInstanceGroups().filter((group) => group.isInvalid());
  }

  getPinnedVersion(): Option<string> {
    return fromGuard(isNonEmptyString, (this.groupConfig as Pinned).pinVersion);
  }

  /** Is `pinVersion` defined and this group does not match that version? */
  isUnpinned(): boolean {
    return pipe(
      this.getPinnedVersion(),
      mapO((pinVersion) =>
        this.instances.some((o) => o.version !== pinVersion),
      ),
      unwrapOr<boolean>(false),
    );
  }
}
