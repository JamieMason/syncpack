import { Effect, pipe } from 'effect';
import { BaseSpecifier } from './base.js';
import { Specifier } from './index.js';
import { NonSemverError } from './lib/non-semver-error.js';
import type { SpecificRegistryResult } from './lib/specific-registry-result.js';

type T = SpecificRegistryResult<'version'>;

/**
 * An exact semver version
 * @example "1.4.4"
 */
export class ExactSpecifier extends BaseSpecifier<T> {
  _tag = 'Exact';

  /** The public name referenced in config */
  name = 'exact' as const;

  /** Return the semver version */
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
