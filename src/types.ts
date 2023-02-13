import type { z } from 'zod';
import type { Context as TContext } from './get-context';
import type { CoreTypeName } from './get-context/get-config/get-core-types';
import type {
  Cli as cli,
  Private as privateSchema,
  Public as publicSchema,
  SyncpackRc as syncpackRc,
} from './get-context/get-config/schema';
import type * as paths from './get-context/get-config/schema/paths';
import type * as semverGroup from './get-context/get-config/schema/semver-group';
import type * as semverRange from './get-context/get-config/schema/semver-range';
import type * as versionGroup from './get-context/get-config/schema/version-group';
import type { SemverGroup as TSemverGroup } from './get-context/get-groups/semver-group';
import type { VersionGroup as TVersionGroup } from './get-context/get-groups/version-group';
import type { PackageJsonFile as TPackageJsonFile } from './get-context/get-package-json-files/package-json-file';
import type { Instance as TInstance } from './get-context/get-package-json-files/package-json-file/instance';

export namespace Syncpack {
  export type Ctx = TContext;
  export type Instance = TInstance;
  export type PackageJsonFile = TPackageJsonFile;
  export type PathDefinition = z.infer<typeof paths.pathDefinition>;
  export type TypeName = CoreTypeName | string;
  export type SemverGroup = TSemverGroup;
  export type VersionGroup = TVersionGroup;

  export namespace Config {
    /** All config which can be set via the command line */
    export type Cli = z.infer<typeof cli>;
    /** @private */
    export type Private = z.infer<typeof privateSchema>;
    /** All config which can be set via the command line and/or .syncpackrc */
    export type Public = z.infer<typeof publicSchema>;
    /** All valid contents of a .syncpackrc */
    export type SyncpackRc = z.infer<typeof syncpackRc>;

    export namespace Paths {
      type T = typeof paths;
      /** Direct syncpack where and how to find and fix versions elsewhere */
      export type ConfigByName = z.infer<T['pathConfigByName']>;
    }

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
      export type Value = z.infer<typeof semverRange.value>;
    }

    export namespace SemverGroup {
      type T = typeof semverGroup;
      /** Let dependencies in this group do whatever they like */
      export type Ignored = z.infer<T['ignored']>;
      /** Ensure the version range of dependencies in this group is always this */
      export type WithRange = z.infer<T['withRange']>;
      /** @private */
      export type Default = z.infer<T['base']>;
      /** Every valid type of SemverGroup */
      export type Any = z.infer<T['any']>;
    }

    export namespace VersionGroup {
      type T = typeof versionGroup;
      /** Partion these dependencies and make sure they match internally */
      export type Standard = z.infer<T['standard']>;
      /** Prevent dependencies in this group from being added to the project */
      export type Banned = z.infer<T['banned']>;
      /** Let dependencies in this group do whatever they like */
      export type Ignored = z.infer<T['ignored']>;
      /** Override the version of dependencies in this group to always be this */
      export type Pinned = z.infer<T['pinned']>;
      /** @private */
      export type Default = z.infer<T['base']>;
      /** Every valid type of VersionGroup */
      export type Any = z.infer<T['any']>;
    }
  }
}
