import { Data, Effect, pipe } from 'effect';
import intersects from 'semver/ranges/intersects.js';
import { uniq } from 'tightrope/array/uniq.js';
import type { VersionGroupConfig } from '../config/types.js';
import type { Ctx } from '../get-context/index.js';
import type { Instance } from '../get-instances/instance.js';
import { Report } from '../report.js';
import { Specifier } from '../specifier/index.js';
import type { WorkspaceProtocolSpecifier } from '../specifier/workspace-protocol.js';
import { groupBy } from './lib/group-by.js';

export class SameRangeVersionGroup extends Data.TaggedClass('SameRange')<{
  ctx: Ctx;
  config: VersionGroupConfig.SameRange;
  instances: Instance[];
}> {
  groupType = 'versionGroup';

  constructor(ctx: Ctx, config: VersionGroupConfig.SameRange) {
    super({
      ctx,
      config,
      instances: [],
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspectAll(): Effect.Effect<never, never, Report.Version.Group[]> {
    return Effect.all(
      Object.entries(groupBy('name', this.instances)).flatMap(([name, instances]) =>
        pipe(
          instances,
          Effect.partition((instance) =>
            pipe(
              Effect.succeed(Specifier.create(instance, instance.rawSpecifier.raw)),
              Effect.flatMap((specifier) =>
                pipe(
                  specifier.getSemver(),
                  Effect.matchEffect({
                    onFailure: () =>
                      Effect.fail(
                        // ✘ expected version is not semver
                        // ✘ is a mismatch we can't auto-fix
                        new Report.UnsupportedMismatch(specifier.instance),
                      ),
                    onSuccess: () =>
                      pipe(
                        specifier.instance.semverGroup.getFixed(specifier),
                        Effect.matchEffect({
                          onFailure: () =>
                            Effect.fail(
                              // ✓ expected version is semver
                              // ✘ expected version is not fixable by its semver group
                              // ✘ is a mismatch we can't auto-fix
                              new Report.UnsupportedMismatch(specifier.instance),
                            ),
                          onSuccess: (valid) =>
                            specifier.instance.rawSpecifier.raw === valid.raw
                              ? Effect.succeed(
                                  // ✓ expected version is semver
                                  // ✓ expected version matches its semver group
                                  // ✓ current version matches expected
                                  new Report.Valid(specifier),
                                )
                              : Effect.fail(
                                  // ✓ expected version is semver
                                  // ✓ expected version matches its semver group
                                  // ✘ current version mismatches expected
                                  // ✓ is a mismatch we can auto-fix
                                  new Report.SemverRangeMismatch(valid),
                                ),
                        }),
                      ),
                  }),
                ),
              ),
            ),
          ),
          Effect.map(([allMismatches, allMatches]) =>
            allMismatches.length === 0
              ? allMatches.map((thisMatch) => {
                  if (thisMatch.specifier.instance.strategy.name === 'local') {
                    // ✓ every instance is valid on its own
                    // ✓ expected version is semver
                    // ! is the original local package
                    // ✓ others must match this, not the other way around
                    return thisMatch;
                  }
                  const mismatches = getRangeMismatches(this.ctx, thisMatch, allMatches);
                  if (mismatches.length === 0) {
                    // ✓ every instance is valid on its own
                    // ✓ expected version is semver
                    // ✓ expected version matches its semver group
                    // ✓ current version matches expected
                    // ! is not the original local package
                    // ✓ current specifier matches every other specifier
                    return thisMatch;
                  }
                  // ✓ every instance is valid on its own
                  // ✓ expected version is semver
                  // ✓ expected version matches its semver group
                  // ✓ current version matches expected
                  // ! is not the original local package
                  // ✘ current specifier does not match every other specifier
                  return new Report.SameRangeMismatch(
                    thisMatch.specifier.instance,
                    uniq(
                      mismatches.map((report) =>
                        String(report.specifier.instance.rawSpecifier.raw),
                      ),
                    ),
                  );
                })
              : // ✘ not every instance is valid on its own
                // ! report on their validity individually ! when all are valid
                //   they can progress to being checked for having compatible
                //   ranges
                [...allMatches, ...allMismatches],
          ),
          Effect.map((reports) => ({ name, reports })),
        ),
      ),
    );
  }
}

/** Find all ranges/versions which this semver version does not cover */
function getRangeMismatches(
  ctx: Ctx,
  report: Report.Valid,
  others: Report.Valid[],
): Report.Valid[] {
  return others.filter((other) => !matchesRange(ctx, report, other));
}

/** Does semver version `a` match semver version `b`? */
function matchesRange(ctx: Ctx, a: Report.Valid, b: Report.Valid): boolean {
  const loose = true;
  return intersects(unwrapSemver(ctx, a.specifier), unwrapSemver(ctx, b.specifier), loose);
}

/** Get the semver version synchronously from a specifier known to contain semver */
function unwrapSemver(ctx: Ctx, specifier: Specifier.Any): string {
  if (specifier._tag === 'Range' || specifier._tag === 'Exact') {
    return specifier.raw as string;
  }
  if (specifier._tag === 'WorkspaceProtocol') {
    return Effect.runSync((specifier as WorkspaceProtocolSpecifier).getSemverEquivalent(ctx));
  }
  return Effect.runSync(specifier.getSemver());
}
