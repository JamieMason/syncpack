import { Data } from 'effect';
import type { Union } from 'ts-toolbelt';
import type { Instance } from './get-instances/instance';
import type { Specifier } from './specifier';

export namespace Report {
  export namespace Semver {
    export type Any = Union.Strict<Report.Semver.Invalid.Any | Report.Semver.Valid.Any>;

    export namespace Valid {
      export type Any = Union.Strict<
        Report.Disabled | Report.FilteredOut | Report.Ignored | Report.Valid
      >;
    }

    export namespace Invalid {
      export type Any = Union.Strict<Report.Semver.Fixable.Any | Report.Semver.Unfixable.Any>;
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

    export type Any = Union.Strict<Report.Version.Invalid.Any | Report.Version.Valid.Any>;

    export namespace Valid {
      export type Any = Union.Strict<
        Report.Disabled | Report.FilteredOut | Report.Ignored | Report.Valid
      >;
    }

    export namespace Invalid {
      export type Any = Union.Strict<Report.Version.Fixable.Any | Report.Version.Unfixable.Any>;
    }

    export namespace Fixable {
      export type Any = Union.Strict<
        | Report.Banned
        | Report.HighestSemverMismatch
        | Report.LocalPackageMismatch
        | Report.LowestSemverMismatch
        | Report.PinnedMismatch
        | Report.SemverRangeMismatch
        | Report.SnappedToMismatch
      >;
    }

    export namespace Unfixable {
      export type Any = Union.Strict<
        | Report.MissingLocalVersion
        | Report.MissingSnappedToMismatch
        | Report.UnsupportedMismatch
        | Report.SameRangeMismatch
      >;
    }
  }

  /** Semver Groups are disabled by default */
  export class Disabled extends Data.TaggedClass('Disabled')<{
    readonly instance: Instance;
  }> {}

  /** Has a name which does not match the `--filter` RegExp */
  export class FilteredOut extends Data.TaggedClass('FilteredOut')<{
    readonly instance: Instance;
  }> {}

  /** Is in an ignored version group */
  export class Ignored extends Data.TaggedClass('Ignored')<{
    readonly instance: Instance;
  }> {}

  /** Version satisfies the rules of its version group */
  export class Valid extends Data.TaggedClass('Valid')<{
    readonly specifier: Specifier.Any;
  }> {}

  /** Must be removed */
  export class Banned extends Data.TaggedClass('Banned')<{
    readonly fixable: Specifier.Any;
  }> {}

  /** Version mismatches and should use a higher version found on another */
  export class HighestSemverMismatch extends Data.TaggedClass('HighestSemverMismatch')<{
    readonly fixable: Specifier.Any;
  }> {}

  /** Version mismatches the `.version` of the package developed in this repo */
  export class LocalPackageMismatch extends Data.TaggedClass('LocalPackageMismatch')<{
    readonly fixable: Specifier.Any;
    readonly localInstance: Instance;
  }> {}

  /** Version mismatches and should use a lower version found on another */
  export class LowestSemverMismatch extends Data.TaggedClass('LowestSemverMismatch')<{
    readonly fixable: Specifier.Any;
  }> {}

  /** Version is not identical to the `pinVersion` of its Pinned version group */
  export class PinnedMismatch extends Data.TaggedClass('PinnedMismatch')<{
    readonly fixable: Specifier.Any;
  }> {}

  /** Version is identical but the semver range does not match its semver group */
  export class SemverRangeMismatch extends Data.TaggedClass('SemverRangeMismatch')<{
    readonly fixable: Specifier.Any;
  }> {}

  /** Version mismatches the version used by the packages in the `snapTo` array */
  export class SnappedToMismatch extends Data.TaggedClass('SnappedToMismatch')<{
    readonly fixable: Specifier.Any;
    readonly localInstance: Instance;
  }> {}

  /** Dependency should match a local package.json which is missing a .version */
  export class MissingLocalVersion extends Data.TaggedClass('MissingLocalVersion')<{
    readonly localInstance: Instance;
    readonly unfixable: Instance;
  }> {}

  /** Dependency is not present in any of the packages in the `snapTo` array */
  export class MissingSnappedToMismatch extends Data.TaggedClass('MissingSnappedToMismatch')<{
    readonly unfixable: Instance;
  }> {}

  /** Specifier does not cover the specifiers of every other instance in this group */
  export class SameRangeMismatch extends Data.TaggedClass('SameRangeMismatch')<{
    readonly unfixable: Instance;
    /** the raw specifiers which this instance's specifier did not cover */
    readonly mismatches: string[];
  }> {}

  /** Version mismatches and is not semver, syncpack cannot guess what to do */
  export class UnsupportedMismatch extends Data.TaggedClass('UnsupportedMismatch')<{
    readonly unfixable: Instance;
  }> {}
}
