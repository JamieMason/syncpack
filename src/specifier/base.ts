import { Effect, pipe } from 'effect';
import type { Instance } from '../get-instances/instance.js';
import { Specifier } from './index.js';
import { NonSemverError } from './lib/non-semver-error.js';
import type { NpmPackageArgResult } from './lib/parse-specifier.js';
import { parseSpecifier } from './lib/parse-specifier.js';

export class BaseSpecifier<T extends NpmPackageArgResult | unknown> {
  /** should be overridden by sub classes */
  _tag = 'Base';

  /** should be overridden by sub classes */
  name = 'base';

  /** The raw semver/workspace:/git etc version value */
  raw: string;

  /**
   * A `Specifier` describes the version specifier (eg "^1.4.4") of a given
   * instance.
   *
   * Initially there will be a `Specifier` which describes the version as it is
   * on disk, but other `Specifier`s will need to be created which reflect what
   * the version should be once fixed – first for the instance's version group
   * and second for its semver group.
   *
   * The intermediate and final variants of `Specifier` could differ along the
   * way and whether we're linting the current state of the monorepo or
   * determining whether possible fixes will ultimately still have a valid
   * version and range, each one has a reference back to the original `Instance`
   * for writing back to it when we do finally commit our fixes once verified.
   */
  instance: Instance;

  constructor(data: Pick<BaseSpecifier<T>, 'raw' | 'instance'>) {
    this.raw = data.raw;
    this.instance = data.instance;
  }

  /**
   * Parse the raw version specifier using
   * https://github.com/npm/npm-package-arg
   */
  protected parse(): Effect.Effect<never, unknown, T> {
    const name = this.instance.name;
    const raw = this.raw;
    const packageJsonFile = this.instance.packageJsonFile;
    return pipe(
      Effect.try(() => parseSpecifier(name, raw, packageJsonFile) as T),
      Effect.tapError(() =>
        Effect.logError(
          `parseSpecifier threw on ${name}@${raw} at ${packageJsonFile.jsonFile.shortPath}`,
        ),
      ),
    );
  }

  /** Return the version portion if it is valid semver */
  getSemver(this: Specifier.Any): Effect.Effect<never, NonSemverError, string> {
    return NonSemverError.asEffect(this);
  }

  /** Get a new `Specifier` from the given semver version applied to this one */
  setSemver(
    this: Specifier.Any,
    _version: string,
  ): Effect.Effect<never, NonSemverError, Specifier.Any> {
    return NonSemverError.asEffect(this);
  }

  /** Apply the given specifier to a new one with this instance bound to it */
  replaceWith<T extends Specifier.Any>(specifier: T): T {
    return Specifier.create(this.instance, specifier.raw) as T;
  }
}
