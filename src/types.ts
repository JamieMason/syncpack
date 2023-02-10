import type { z } from 'zod';
import type { Context as TContext } from './lib/get-context';
import type {
  Cli as CliSchema,
  Private as PrivateSchema,
  Public as PublicSchema,
  SyncpackRc as SyncpackRcSchema,
} from './lib/get-context/get-config/schema';
import type * as DependencyTypeSchema from './lib/get-context/get-config/schema/dependency-type';
import type * as SemverGroupSchema from './lib/get-context/get-config/schema/semver-group';
import type * as SemverRangeSchema from './lib/get-context/get-config/schema/semver-range';
import type * as VersionGroupSchema from './lib/get-context/get-config/schema/version-group';
import type { SemverGroup as TSemverGroup } from './lib/get-context/get-groups/semver-group';
import type { VersionGroup as TVersionGroup } from './lib/get-context/get-groups/version-group';
import type { PackageJsonFile as TPackageJsonFile } from './lib/get-context/get-package-json-files/package-json-file';
import type { Instance as TInstance } from './lib/get-context/get-package-json-files/package-json-file/instance';

export namespace Syncpack {
  export type Ctx = TContext;
  export type Instance = TInstance;
  export type PackageJsonFile = TPackageJsonFile;
  export type VersionGroup = TVersionGroup;
  export type SemverGroup = TSemverGroup;

  export namespace Config {
    /** All config which can be set via the command line */
    export type Cli = z.infer<typeof CliSchema>;
    /** @private */
    export type Private = z.infer<typeof PrivateSchema>;
    /** All config which can be set via the command line and/or .syncpackrc */
    export type Public = z.infer<typeof PublicSchema>;
    /** All valid contents of a .syncpackrc */
    export type SyncpackRc = z.infer<typeof SyncpackRcSchema>;

    export namespace SemverRange {
      /**
       * Aliases for semver range formats supported by syncpack
       *
       * Defaults to `""` to ensure that exact dependency versions are used
       * instead of loose ranges, but this can be overridden in your config file
       * or via the `--semver-range` command line option.
       *
       * | Supported Range |   Example |
       * | --------------- | --------: |
       * | `"<"`           |  `<1.4.2` |
       * | `"<="`          | `<=1.4.2` |
       * | `""`            |   `1.4.2` |
       * | `"~"`           |  `~1.4.2` |
       * | `"^"`           |  `^1.4.2` |
       * | `">="`          | `>=1.4.2` |
       * | `">"`           |  `>1.4.2` |
       * | `"*"`           |       `*` |
       *
       * @default ""
       */
      export type Value = z.infer<typeof SemverRangeSchema.Value>;
    }

    export namespace SemverGroup {
      /** Let dependencies in this group do whatever they like */
      export type Ignored = z.infer<typeof SemverGroupSchema.Ignored>;
      /** Ensure the version range of dependencies in this group is always this */
      export type WithRange = z.infer<typeof SemverGroupSchema.WithRange>;
      /** @private */
      export type Default = z.infer<typeof SemverGroupSchema.Default>;
      /** Every valid type of SemverGroup */
      export type Any = z.infer<typeof SemverGroupSchema.Any>;
    }

    export namespace VersionGroup {
      /** Partion these dependencies and make sure they match internally */
      export type Standard = z.infer<typeof VersionGroupSchema.Standard>;
      /** Prevent dependencies in this group from being added to the project */
      export type Banned = z.infer<typeof VersionGroupSchema.Banned>;
      /** Let dependencies in this group do whatever they like */
      export type Ignored = z.infer<typeof VersionGroupSchema.Ignored>;
      /** Override the version of dependencies in this group to always be this */
      export type Pinned = z.infer<typeof VersionGroupSchema.Pinned>;
      /** @private */
      export type Default = z.infer<typeof VersionGroupSchema.Default>;
      /** Every valid type of VersionGroup */
      export type Any = z.infer<typeof VersionGroupSchema.Any>;
    }

    export namespace DependencyType {
      /** Alias for paths to version properties in package.json files */
      export type Name = z.infer<typeof DependencyTypeSchema.Name>;
      /** Array of paths to version properties in package.json files */
      export type NameList = z.infer<typeof DependencyTypeSchema.NameList>;
      /** The isEnabled status of each DependencyType by name */
      export type Flags = z.infer<typeof DependencyTypeSchema.Flags>;
    }
  }
}
