import * as Data from '@effect/data/Data';
import { pipe } from '@effect/data/Function';
import * as Option from '@effect/data/Option';
import type npa from 'npm-package-arg';
import type {
  AliasResult,
  FileResult,
  HostedGitResult,
  RegistryResult,
  URLResult,
} from 'npm-package-arg';
import type { Strategy } from '../config/get-custom-types';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { DELETE, type Delete } from '../get-version-groups/lib/delete';
import { $R } from '../lib/$R';

/** Extends npm/npm-package-arg to support "workspace:*" */
export interface WorkspaceProtocolResult {
  type: 'workspaceProtocol';
  raw: string;
  name: string | null;
  escapedName: string | null;
  scope: string | null;
  rawSpec: string;
  saveSpec: string;
}

export type NpmPackageArgResult = ReturnType<typeof npa.resolve> | WorkspaceProtocolResult;

interface BaseInstance<T extends NpmPackageArgResult | unknown> {
  /** the name of this dependency */
  name: string;
  /** The package this dependency is installed in this specific time */
  packageJsonFile: PackageJsonFile;
  /** @see https://github.com/npm/npm-package-arg */
  parsedSpecifier: T;
  /** The .name property of the package.json file of this instance */
  pkgName: string;
  /** The raw: semver, git, workspace: etc version value */
  specifier: string;
  /** locates where in the file this dependency is installed */
  strategy: Strategy.Any;
}

export namespace Instance {
  /**
   * A helper to create specific classes for each of the possible
   * `RegistryResult` types from npm/npm-package-arg. Instead of grouping them
   * together we are being more specific
   */
  type SpecificRegistryResult<T extends RegistryResult['type'] | 'localPackage'> = Omit<
    RegistryResult,
    'type'
  > & {
    type: T;
  };

  /** An `Instance` whose specifier is a path to a local file or directory */
  export class FileInstance extends Data.TaggedClass('FileInstance')<BaseInstance<FileResult>> {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      return Option.none();
    }
  }

  /** An `Instance` whose specifier is a Git URL */
  export class HostedGitInstance extends Data.TaggedClass('HostedGitInstance')<
    BaseInstance<HostedGitResult>
  > {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      // @TODO: If git tag is semver, return that
      return Option.none();
    }
  }

  /** An `Instance` whose specifier is a URL to a tarball */
  export class UrlInstance extends Data.TaggedClass('URLInstance')<BaseInstance<URLResult>> {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      // @TODO: If file name is semver, return that
      return Option.none();
    }
  }

  /** An `Instance` whose specifier is eg "npm:imageoptim-cli@3.1.7" */
  export class AliasInstance extends Data.TaggedClass('AliasInstance')<BaseInstance<AliasResult>> {
    setSpecifier(version: string | Delete): void {
      if (version === DELETE) {
        setSpecifier.call(this, version);
      } else {
        const subSpec = this.parsedSpecifier.subSpec;
        const name = subSpec.name || '';
        const specifier = `npm:${name}@${version}`;
        setSpecifier.call(this, specifier);
      }
    }

    getSemverSpecifier(): Option.Option<string> {
      const subSpec = this.parsedSpecifier.subSpec;
      if (['range', 'version'].includes(subSpec.type) && subSpec.fetchSpec !== null) {
        return Option.some(subSpec.fetchSpec);
      }
      return Option.none();
    }
  }

  /** An `Instance` whose specifier is exact semver */
  export class VersionInstance extends Data.TaggedClass('VersionInstance')<
    BaseInstance<SpecificRegistryResult<'version'>>
  > {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      return Option.some(this.parsedSpecifier.fetchSpec);
    }
  }

  /** An `Instance` whose specifier is eg. "*" or "^1.2.3" */
  export class RangeInstance extends Data.TaggedClass('RangeInstance')<
    BaseInstance<SpecificRegistryResult<'range'>>
  > {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      return Option.some(this.parsedSpecifier.fetchSpec);
    }
  }

  /** An `Instance` whose specifier is eg. "latest" or "made-up-by-some-dev" */
  export class TagInstance extends Data.TaggedClass('TagInstance')<
    BaseInstance<SpecificRegistryResult<'tag'>>
  > {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      return Option.none();
    }
  }

  /** An `Instance` whose specifier is "workspace:*" */
  export class WorkspaceProtocolInstance extends Data.TaggedClass('WorkspaceProtocolInstance')<
    BaseInstance<WorkspaceProtocolResult>
  > {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      return Option.none();
    }
  }

  /** An `Instance` whose specifier is not supported by npm */
  export class UnsupportedInstance extends Data.TaggedClass('UnsupportedInstance')<
    BaseInstance<unknown>
  > {
    setSpecifier = setSpecifier;

    getSemverSpecifier(): Option.Option<string> {
      return Option.none();
    }
  }

  export type Any =
    | FileInstance
    | HostedGitInstance
    | UrlInstance
    | AliasInstance
    | VersionInstance
    | RangeInstance
    | TagInstance
    | WorkspaceProtocolInstance
    | UnsupportedInstance;
}

/**
 * In the case of banned dependencies, their version is set to `undefined`,
 * which causes them to be removed by `JSON.stringify`.
 */
function setSpecifier(this: Instance.Any, version: string | Delete): void {
  const file = this.packageJsonFile;
  pipe(this.strategy.write(file, [this.name, version]), $R.tapErrVerbose);
}
