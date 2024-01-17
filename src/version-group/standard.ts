import { Data, Effect, pipe } from 'effect';
import { uniq } from 'tightrope/array/uniq.js';
import type { VersionGroupConfig } from '../config/types.js';
import type { Instance } from '../get-instances/instance.js';
import { Report } from '../report.js';
import { Specifier } from '../specifier/index.js';
import { getPreferredVersion } from './lib/get-preferred-version.js';
import { groupBy } from './lib/group-by.js';

export class StandardVersionGroup extends Data.TaggedClass('Standard')<{
  config: VersionGroupConfig.Standard;
  instances: Instance[];
  isCatchAll: boolean;
}> {
  groupType = 'versionGroup';

  constructor(isCatchAll: boolean, config: VersionGroupConfig.Standard) {
    super({
      config,
      instances: [],
      isCatchAll,
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspectAll(): Effect.Effect<never, never, Report.Version.Group[]> {
    return Effect.all(
      Object.entries(groupBy('name', this.instances)).flatMap(([name, instances]) => {
        const localInstance = getLocalInstance(instances);

        if (localInstance) {
          const localVersion = localInstance?.rawSpecifier.raw;
          return pipe(
            Effect.succeed(Specifier.create(localInstance, localVersion)),
            Effect.flatMap((local) =>
              Effect.all(
                local._tag !== 'Exact' && instances.length > 1
                  ? instances.map((instance) =>
                      // ! dependency is a package developed in this repo
                      // ✘ local package has an invalid .version property
                      // ✘ is a mismatch we can't auto-fix
                      Effect.succeed(new Report.MissingLocalVersion(instance, localInstance)),
                    )
                  : instances.flatMap((instance) =>
                      // instances.flatMap((instance) =>
                      pipe(
                        Effect.succeed(Specifier.create(instance, instance.rawSpecifier.raw)),
                        Effect.flatMap((specifier) =>
                          specifier.instance === localInstance
                            ? // ✓ this is the local package which the others should match
                              // ! its version must always remain as exact semver
                              // ! other instances need to be adjusted for their semver groups
                              Effect.succeed(new Report.Valid(specifier))
                            : pipe(
                                specifier.replaceWith(local),
                                specifier.instance.semverGroup.getFixed,
                                Effect.match({
                                  onFailure: /* istanbul ignore next */ () =>
                                    // ! is not the local package instance
                                    // ✘ local version is not fixable by this semver group
                                    // ✘ is a mismatch we can't auto-fix
                                    // ✘ this should be impossible - we already proved the local version is exact semver
                                    new Report.UnsupportedMismatch(specifier.instance),
                                  onSuccess: (valid) =>
                                    specifier.instance.rawSpecifier.raw === valid.raw
                                      ? // ! is not the local package instance
                                        // ✓ local version matches this semver group
                                        // ✓ current version matches local
                                        new Report.Valid(specifier)
                                      : localVersion === 'PACKAGE_JSON_HAS_NO_VERSION'
                                        ? // ! is not the local package instance
                                          // ✘ local package has a version defined
                                          // ✓ local version matches this semver group
                                          // ✘ current version mismatches local
                                          new Report.MissingLocalVersion(
                                            specifier.instance,
                                            localInstance,
                                          )
                                        : // ! is not the local package instance
                                          // ✓ local package has a version defined
                                          // ✓ local version matches this semver group
                                          // ✘ current version mismatches local
                                          new Report.LocalPackageMismatch(valid, localInstance),
                                }),
                              ),
                        ),
                      ),
                    ),
              ),
            ),
            Effect.map((reports) => ({ name, reports })),
          );
        }

        const PreferredMismatch =
          this.config.preferVersion === 'lowestSemver'
            ? Report.LowestSemverMismatch
            : Report.HighestSemverMismatch;

        return pipe(
          Effect.succeed(
            instances.map((instance) => Specifier.create(instance, instance.rawSpecifier.raw)),
          ),
          Effect.flatMap((specifiers) =>
            pipe(
              getPreferredVersion(this.config.preferVersion, specifiers),
              Effect.matchEffect({
                onFailure: () =>
                  Effect.succeed(
                    uniq(specifiers.map((specifier) => specifier.instance.rawSpecifier.raw))
                      .length === 1
                      ? specifiers.map(
                          (specifier) =>
                            // ✘ not every version is semver
                            // ✓ every version is identical
                            // ✓ is a match
                            new Report.Valid(specifier),
                        )
                      : instances.map(
                          (instance) =>
                            // ✘ not every version is semver
                            // ✘ some versions are not identical
                            // ✘ is a mismatch we can't auto-fix
                            new Report.UnsupportedMismatch(instance),
                        ),
                  ),
                onSuccess: (expectedVersion) =>
                  pipe(
                    specifiers,
                    Effect.forEach((current) =>
                      pipe(
                        current.replaceWith(expectedVersion),
                        current.instance.semverGroup.getFixed,
                        Effect.match({
                          onFailure: /* istanbul ignore next */ () =>
                            // ✓ every version is semver
                            // ✘ expected version is not fixable by its semver group
                            // ✘ is a mismatch we can't auto-fix
                            // ✘ this should be impossible - any valid semver is fixable by a semver group
                            new Report.UnsupportedMismatch(current.instance),
                          onSuccess: (expectedRange) =>
                            current.instance.rawSpecifier.raw === expectedRange.raw
                              ? // ✓ every version is semver
                                // ✓ current version matches expected semver
                                // ✓ current version matches expected version
                                new Report.Valid(current)
                              : current.instance.rawSpecifier.raw === expectedVersion.raw
                                ? // ✓ every version is semver
                                  // ✓ current version matches expected version
                                  // ✘ current version does not match expected semver
                                  // ✓ is a mismatch we can auto-fix
                                  new Report.SemverRangeMismatch(expectedRange)
                                : // ✓ every version is semver
                                  // ✘ current version does not match expected version
                                  // ✘ expected version does not match expected semver
                                  // ✓ is a mismatch we can auto-fix
                                  new PreferredMismatch(expectedRange),
                        }),
                      ),
                    ),
                  ),
              }),
            ),
          ),
          Effect.map((reports) => ({ name, reports })),
        );
      }),
    );
  }
}

/**
 * If this dependency is developed in this monorepo, get the instance which
 * represents the canonical .version property of its package.json file.
 */
function getLocalInstance(instances: Instance[]): Instance | undefined {
  return instances.find(isLocalInstance);
}

/** Is this dependency developed in this monorepo */
function isLocalInstance(instance: Instance): boolean {
  return instance.strategy.name === 'local';
}
