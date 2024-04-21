import { Data, Effect } from 'effect';
import type { Specifier } from '../index.js';

export class NonSemverError extends Data.TaggedClass('NonSemverError')<{
  specifier: Specifier.Any;
}> {
  static asEffect<T>(specifier: Specifier.Any): Effect.Effect<T, NonSemverError> {
    return Effect.fail(new NonSemverError({ specifier }));
  }
}
