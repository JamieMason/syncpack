import type { Instance } from './get-instances/instance.js';
import type { Specifier } from './specifier/index.js';

export namespace Report {
  export type Any = Semver.Any | Version.Any;

  export namespace Semver {
    export type Any = Report.Semver.Invalid.Any | Report.Semver.Valid.Any;

    export namespace Valid {
      export type Any = Report.Disabled | Report.FilteredOut | Report.Ignored | Report.Valid;
    }

    export namespace Invalid {
      export type Any = Report.Semver.Fixable.Any | Report.Semver.Unfixable.Any;
    }

    export namespace Fixable {
      export type Any = Report.SemverRangeMismatch;
    }

    export namespace Unfixable {
      export type Any = Report.UnsupportedMismatch;
    }
  }

  export namespace Version {
    export interface Group {
      name: string;
      reports: Report.Version.Any[];
    }

    export type Any = Report.Version.Invalid.Any | Report.Version.Valid.Any;

    export namespace Valid {
      export type Any = Report.Disabled | Report.FilteredOut | Report.Ignored | Report.Valid;
    }

    export namespace Invalid {
      export type Any = Report.Version.Fixable.Any | Report.Version.Unfixable.Any;
    }

    export namespace Fixable {
      export type Any =
        | Report.Banned
        | Report.HighestSemverMismatch
        | Report.LocalPackageMismatch
        | Report.LowestSemverMismatch
        | Report.PinnedMismatch
        | Report.SemverRangeMismatch
        | Report.SnappedToMismatch;
    }

    export namespace Unfixable {
      export type Any =
        | Report.MissingLocalVersion
        | Report.MissingSnappedToMismatch
        | Report.UnsupportedMismatch
        | Report.SameRangeMismatch;
    }
  }

  class Excluded {
    readonly _tagGroup = 'Excluded';
    readonly isInvalid = false;
    readonly instance: Instance;

    constructor(instance: Instance) {
      this.instance = instance;
    }
  }

  class Unfixable {
    readonly _tagGroup = 'Unfixable';
    readonly isInvalid = true;
    readonly unfixable: Instance;

    constructor(unfixable: Instance) {
      this.unfixable = unfixable;
    }
  }

  class Fixable {
    readonly _tagGroup = 'Fixable';
    readonly isInvalid = true;
    readonly fixable: Specifier.Any;

    constructor(fixable: Specifier.Any) {
      this.fixable = fixable;
    }
  }

  /** Semver Groups are disabled by default */
  export class Disabled extends Excluded {
    readonly _tag = 'Disabled';
  }

  /** Has a name which does not match the `--filter` RegExp */
  export class FilteredOut extends Excluded {
    readonly _tag = 'FilteredOut';
  }

  /** Is in an ignored version group */
  export class Ignored extends Excluded {
    readonly _tag = 'Ignored';
  }

  /** Version satisfies the rules of its version group */
  export class Valid {
    readonly _tag = 'Valid';
    readonly _tagGroup = 'Valid';
    readonly isInvalid = false;
    readonly specifier: Specifier.Any;

    constructor(specifier: Specifier.Any) {
      this.specifier = specifier;
    }
  }

  /** Must be removed */
  export class Banned extends Fixable {
    readonly _tag = 'Banned';
  }

  /** Version mismatches and should use a higher version found on another */
  export class HighestSemverMismatch extends Fixable {
    readonly _tag = 'HighestSemverMismatch';
  }

  /** Version mismatches the `.version` of the package developed in this repo */
  export class LocalPackageMismatch extends Fixable {
    readonly _tag = 'LocalPackageMismatch';
    readonly localInstance: Instance;

    constructor(fixable: Specifier.Any, localInstance: Instance) {
      super(fixable);
      this.localInstance = localInstance;
    }
  }

  /** Version mismatches and should use a lower version found on another */
  export class LowestSemverMismatch extends Fixable {
    readonly _tag = 'LowestSemverMismatch';
  }

  /** Version is not identical to the `pinVersion` of its Pinned version group */
  export class PinnedMismatch extends Fixable {
    readonly _tag = 'PinnedMismatch';
  }

  /** Version is identical but the semver range does not match its semver group */
  export class SemverRangeMismatch extends Fixable {
    readonly _tag = 'SemverRangeMismatch';
  }

  /** Version mismatches the version used by the packages in the `snapTo` array */
  export class SnappedToMismatch extends Fixable {
    readonly _tag = 'SnappedToMismatch';
    readonly localInstance: Instance;

    constructor(fixable: Specifier.Any, localInstance: Instance) {
      super(fixable);
      this.localInstance = localInstance;
    }
  }

  /** Dependency should match a local package.json which is missing a .version */
  export class MissingLocalVersion extends Unfixable {
    readonly _tag = 'MissingLocalVersion';
    readonly localInstance: Instance;

    constructor(unfixable: Instance, localInstance: Instance) {
      super(unfixable);
      this.localInstance = localInstance;
    }
  }

  /** Dependency is not present in any of the packages in the `snapTo` array */
  export class MissingSnappedToMismatch extends Unfixable {
    readonly _tag = 'MissingSnappedToMismatch';
  }

  /** Specifier does not cover the specifiers of every other instance in this group */
  export class SameRangeMismatch extends Unfixable {
    readonly _tag = 'SameRangeMismatch';
    /** the raw specifiers which this instance's specifier did not cover */
    readonly mismatches: string[];

    constructor(unfixable: Instance, mismatches: string[]) {
      super(unfixable);
      this.mismatches = mismatches;
    }
  }

  /** Version mismatches and is not semver, syncpack cannot guess what to do */
  export class UnsupportedMismatch extends Unfixable {
    readonly _tag = 'UnsupportedMismatch';
  }
}
