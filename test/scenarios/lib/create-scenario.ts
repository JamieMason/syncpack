import * as Context from '@effect/data/Context';
import { pipe } from '@effect/data/Function';
import { unify } from '@effect/data/Unify';
import * as Effect from '@effect/io/Effect';
import { minimatch } from 'minimatch';
import { join, normalize } from 'path';
import { CliConfigTag } from '../../../src/config/tag';
import { CWD } from '../../../src/constants';
import type { SemverRangeEffects, VersionEffects } from '../../../src/create-program/effects';
import { createEnv } from '../../../src/env/create-env';
import { EnvTag } from '../../../src/env/tags';
import type { ErrorHandlers } from '../../../src/error-handlers/create-error-handlers';
import type { Ctx } from '../../../src/get-context';
import { getContext } from '../../../src/get-context';
import type { JsonFile } from '../../../src/get-package-json-files/get-patterns/read-json-safe';
import type { PackageJson } from '../../../src/get-package-json-files/package-json-file';
import type { SemverGroupReport } from '../../../src/get-semver-groups';
import { getSemverGroups } from '../../../src/get-semver-groups';
import type { VersionGroupReport } from '../../../src/get-version-groups';
import { getVersionGroups } from '../../../src/get-version-groups';
import type { MockEnv } from '../../mock-env';
import {
  createMockEnv,
  createMockErrorHandlers,
  createMockSemverRangeEffects,
  createMockVersionEffects,
} from '../../mock-env';

interface MockedFile {
  absolutePath: string;
  after: JsonFile<PackageJson>;
  before: JsonFile<PackageJson>;
  diskWriteWhenChanged: [string, string];
  id: string;
  relativePath: string;
}

export interface TestScenario {
  config: Ctx['config'];
  env: MockEnv;
  errorHandlers: ErrorHandlers<jest.Mock<any, any>>;
  files: {
    'packages/a/package.json': MockedFile;
    'packages/api/package.json': MockedFile;
    'packages/app/package.json': MockedFile;
    'packages/b/package.json': MockedFile;
    'packages/c/package.json': MockedFile;
    'packages/shared/package.json': MockedFile;
    'workspaces/a/packages/a/package.json': MockedFile;
    'workspaces/b/packages/b/package.json': MockedFile;
    'workspaces/b/packages/c/package.json': MockedFile;
  };
  log: jest.SpyInstance;
  report: {
    semverGroups: SemverGroupReport.Any[][];
    versionGroups: VersionGroupReport.Any[][];
  };
  semverEffects: SemverRangeEffects<void>;
  versionEffects: VersionEffects<void>;
}

export function createScenario(
  fileMocks: {
    path: string;
    before: JsonFile<PackageJson>;
    after: JsonFile<PackageJson>;
  }[],
  config: Ctx['config'],
): TestScenario {
  jest.clearAllMocks();
  const env = createMockEnv();
  const semverEffects = createMockSemverRangeEffects();
  const versionEffects = createMockVersionEffects();
  const errorHandlers = createMockErrorHandlers();
  const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
  // resolve all paths
  const mockedFiles: MockedFile[] = fileMocks.map((file) => {
    const absolutePath = join(CWD, file.path);
    const relativePath = normalize(file.path);
    return {
      absolutePath,
      after: {
        ...file.after,
        filePath: absolutePath,
      },
      before: {
        ...file.before,
        filePath: absolutePath,
      },
      diskWriteWhenChanged: [expect.stringContaining(relativePath), file.after.json],
      id: file.path,
      relativePath,
    };
  });
  // mock rcfile
  env.readConfigFileSync.mockImplementation(() => {
    return config.rcFile;
  });
  // mock file system
  env.readFileSync.mockImplementation((filePath): string | undefined => {
    return mockedFiles.find((file) => {
      return normalize(filePath) === normalize(file.absolutePath);
    })?.before?.json;
  });
  // mock globs
  env.globSync.mockImplementation((patterns: string[]): string[] => {
    return patterns.flatMap((pattern) =>
      mockedFiles
        .filter((file) => {
          return minimatch(normalize(file.absolutePath), toPosix(join(CWD, pattern)));
        })
        .map((file) => normalize(file.absolutePath)),
    );
  });
  // create reports
  return Effect.runSync(
    pipe(
      Effect.Do,
      Effect.bind('ctx', () => getContext()),
      Effect.bind('semverGroups', ({ ctx }) => getSemverGroups(ctx)),
      Effect.bind('versionGroups', ({ ctx }) => getVersionGroups(ctx)),
      Effect.bind('semverGroupsReport', ({ semverGroups }) =>
        Effect.succeed(
          semverGroups.map((group) =>
            Effect.runSync(
              Effect.all(
                group
                  .inspect()
                  .map((report) => pipe(unify(report), Effect.catchAll(Effect.succeed))),
              ),
            ),
          ),
        ),
      ),
      Effect.bind('versionGroupsReport', ({ versionGroups }) =>
        Effect.succeed(
          versionGroups.map((group) =>
            Effect.runSync(
              Effect.all(
                group
                  .inspect()
                  .map((report) => pipe(unify(report), Effect.catchAll(Effect.succeed))),
              ),
            ),
          ),
        ),
      ),
      Effect.map(({ versionGroupsReport, semverGroupsReport }) => {
        return {
          config,
          env,
          errorHandlers,
          semverEffects,
          versionEffects,
          log,
          files: mockedFiles.reduce((memo, file) => {
            memo[file.id] = file;
            return memo;
          }, {} as any),
          report: {
            semverGroups: semverGroupsReport,
            versionGroups: versionGroupsReport,
          },
        };
      }),
      Effect.provideContext(
        pipe(
          Context.empty(),
          Context.add(CliConfigTag, config.cli),
          Context.add(EnvTag, createEnv(env)),
        ),
      ),
    ),
  );
}

function toPosix(value: string): string {
  return value.replace('C:', '').replace(/\\/g, '/');
}
