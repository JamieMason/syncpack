import { isSemver } from '../../lib/is-semver';
import { setSemverRange } from '../../lib/set-semver-range';
import type { Syncpack } from '../../types';
import type { Instance } from '../get-package-json-files/package-json-file/instance';
import { BaseGroup } from './base-group';

type WithRange = Syncpack.Config.SemverGroup.WithRange;
type Ignored = Syncpack.Config.SemverGroup.Ignored;
type MismatchesByName = [string, Instance[]];

export class SemverGroup extends BaseGroup<Syncpack.Config.SemverGroup.Any> {
  constructor(
    config: Syncpack.Config.Private,
    semverGroup: Syncpack.Config.SemverGroup.Any,
  ) {
    super(config, semverGroup);
  }

  getExpectedVersion(instance: Instance): string {
    const version = instance.version;
    // leave ignored versions alone
    if (this.isIgnored()) return version;
    // leave unsupported versions alone
    if (!isSemver(version)) return version;
    // version property of package.json must always be exact
    if (instance.isWorkspace()) return setSemverRange('', version);
    // otherwise we can change it
    const range = (this.groupConfig as WithRange).range;
    return setSemverRange(range, version);
  }

  /** Does this `Instance` have a version which does not follow the rules? */
  isMismatch(instance: Instance): boolean {
    return instance.version !== this.getExpectedVersion(instance);
  }

  /** 1+ `Instance` has a version which does not follow the rules */
  hasMismatches() {
    return this.getMismatches().length > 0;
  }

  isIgnored() {
    return (this.groupConfig as Ignored).isIgnored === true;
  }

  /** Get every `Instance` with a version which does not follow the rules */
  getMismatches(): MismatchesByName[] {
    return this.isIgnored()
      ? []
      : Object.entries(this.instancesByName)
          .map<MismatchesByName>(([name, instances]) => [
            name,
            instances.filter((instance) => this.isMismatch(instance)),
          ])
          .filter(([, arr]) => arr.length > 0);
  }
}
