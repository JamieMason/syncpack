import { Effect, pipe } from 'effect';
import type { HostedGitResult } from 'npm-package-arg';
import { isSemver } from '../guards/is-semver.js';
import { BaseSpecifier } from './base.js';
import { Specifier } from './index.js';
import { NonSemverError } from './lib/non-semver-error.js';

/** @example "git+https://github.com/user/foo" */
export class HostedGitSpecifier extends BaseSpecifier<HostedGitResult> {
  _tag = 'HostedGit';

  /** The public name referenced in config */
  name = 'hosted-git' as const;

  /** Return the git tag if it is valid semver */
  getSemver(): Effect.Effect<never, NonSemverError, string> {
    return pipe(
      this.parse(),
      Effect.mapError(() => new NonSemverError({ specifier: this })),
      Effect.map((parsed) => parsed.gitCommittish || ''),
      Effect.flatMap((gitCommittish) =>
        isSemver(gitCommittish) ? Effect.succeed(gitCommittish) : NonSemverError.asEffect(this),
      ),
    );
  }

  /** Get a new `Specifier` from the given semver version applied to this one */
  setSemver(version: string): Effect.Effect<never, NonSemverError, Specifier.Any> {
    return pipe(
      this.parse(),
      Effect.mapError(() => new NonSemverError({ specifier: this })),
      Effect.map((parsed) => ({
        gitCommittish: parsed.gitCommittish || '',
        rawSpec: parsed.rawSpec || '',
      })),
      Effect.map(({ gitCommittish, rawSpec }) => rawSpec.replace(gitCommittish, version)),
      Effect.map((raw) => Specifier.create(this.instance, raw)),
    );
  }
}
