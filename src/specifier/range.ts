import { Effect, pipe } from 'effect';
import { BaseSpecifier } from './base.js';
import { Specifier } from './index.js';
import { NonSemverError } from './lib/non-semver-error.js';
import type { SpecificRegistryResult } from './lib/specific-registry-result.js';

type T = SpecificRegistryResult<'range'>;

/**
 * @example "^1.2.3"
 */
export class RangeSpecifier extends BaseSpecifier<T> {
  _tag = 'Range';

  /** The public name referenced in config */
  name = 'range' as const;

  /** Return the semver version including the range */
  getSemver(): Effect.Effect<never, NonSemverError, string> {
    return pipe(
      this.parse(),
      Effect.mapError(() => new NonSemverError({ specifier: this })),
      Effect.map((parsed) => parsed.fetchSpec),
    );
  }

  /** Get a new `Specifier` from the given semver version applied to this one */
  setSemver(version: string): Effect.Effect<never, never, Specifier.Any> {
    return Effect.succeed(Specifier.create(this.instance, version));
  }
}
