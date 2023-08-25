import { Effect, flow, pipe } from 'effect';
import { getEnabledTypes } from '../config/get-enabled-types';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import type { Ctx } from '../get-context';
import { canAddToGroup } from '../guards/can-add-to-group';
import type { Io } from '../io';
import { sortByName } from '../lib/sort-by-name';
import type { SemverGroup } from '../semver-group';
import { createSemverGroups } from '../semver-group/create-semver-groups';
import type { VersionGroup } from '../version-group';
import { createVersionGroups } from '../version-group/create-version-groups';
import type { Instance } from './instance';

interface Instances {
  all: Instance[];
  semverGroups: SemverGroup.Any[];
  versionGroups: VersionGroup.Any[];
}

export function getInstances(
  ctx: Ctx,
  io: Io,
  errorHandlers: ErrorHandlers,
): Effect.Effect<never, never, Instances> {
  const exitOnError = Effect.flatMap(() => Effect.failSync(() => io.process.exit(1)));
  return pipe(
    Effect.Do,
    Effect.bind('enabledTypes', () => getEnabledTypes(ctx.config)),
    Effect.bind('semverGroups', () => createSemverGroups(ctx)),
    Effect.bind('versionGroups', () => createVersionGroups(ctx)),
    Effect.bind('instances', (acc) =>
      pipe(
        ctx.packageJsonFiles,
        Effect.forEach((file) => file.getInstances(acc.enabledTypes)),
        Effect.map((instancesByFile) => instancesByFile.flat()),
      ),
    ),
    Effect.tap(({ instances, semverGroups, versionGroups }) =>
      Effect.sync(() => {
        for (const instance of instances) {
          // assign each instance to its semver group, first match wins
          assignToSemverGroup: for (const group of semverGroups) {
            if (canAddToGroup(ctx.packageJsonFilesByName, group, instance)) {
              instance.semverGroup = group;
              group.instances.push(instance);
              break assignToSemverGroup;
            }
          }
          // assign each instance to its version group, first match wins
          assignToVersionGroup: for (const group of versionGroups) {
            if (canAddToGroup(ctx.packageJsonFilesByName, group, instance)) {
              instance.versionGroup = group;
              group.instances.push(instance);
              break assignToVersionGroup;
            }
          }
        }
      }),
    ),
    Effect.map(({ instances, semverGroups, versionGroups }) => ({
      all: instances,
      semverGroups: getSortedAndFiltered(semverGroups),
      versionGroups: getSortedAndFiltered(versionGroups),
    })),
    Effect.catchTags({
      DeprecatedTypesError: flow(errorHandlers.DeprecatedTypesError, exitOnError),
      InvalidCustomTypeError: flow(errorHandlers.InvalidCustomTypeError, exitOnError),
      RenamedWorkspaceTypeError: flow(errorHandlers.RenamedWorkspaceTypeError, exitOnError),
      SemverGroupConfigError: flow(errorHandlers.SemverGroupConfigError, exitOnError),
      VersionGroupConfigError: flow(errorHandlers.VersionGroupConfigError, exitOnError),
    }),
  );

  function getSortedAndFiltered<T extends SemverGroup.Any | VersionGroup.Any>(groups: T[]) {
    return groups.filter((group) => group.instances.sort(sortByName).length > 0);
  }
}
