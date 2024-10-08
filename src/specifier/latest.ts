import { Effect, pipe } from 'effect';
import { BaseSpecifier } from './base.js';
import { Specifier } from './index.js';
import { NonSemverError } from './lib/non-semver-error.js';
import type { SpecificRegistryResult } from './lib/specific-registry-result.js';

type T = SpecificRegistryResult<'range'>;

/**
 * @example "*"
 */
export class LatestSpecifier extends BaseSpecifier<T> {
  _tag = 'Latest';

  /** The public name referenced in config */
  name = 'latest' as const;

  /** Return the semver version including the range */
  getSemver(): Effect.Effect<string, NonSemverError> {
    return pipe(
      this.parse(),
      Effect.mapError(() => new NonSemverError({ specifier: this })),
      Effect.map(parsed => parsed.fetchSpec),
    );
  }

  /** Get a new `Specifier` from the given semver version applied to this one */
  setSemver(version: string): Effect.Effect<Specifier.Any> {
    return Effect.succeed(Specifier.create(this.instance, version));
  }
}
