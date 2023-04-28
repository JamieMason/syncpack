import minimatch from 'minimatch';
import { join, normalize } from 'path';
import { CWD } from '../../../src/constants';
import type { Context } from '../../../src/get-context';
import { getContext } from '../../../src/get-context';
import type { JsonFile } from '../../../src/get-package-json-files/get-patterns/read-json-safe';
import type { PackageJson } from '../../../src/get-package-json-files/package-json-file';
import type { SemverGroupReport } from '../../../src/get-semver-groups';
import { getSemverGroups } from '../../../src/get-semver-groups';
import type { VersionGroupReport } from '../../../src/get-version-groups';
import { getVersionGroups } from '../../../src/get-version-groups';
import type { MockDisk } from '../../mock-disk';
import { mockDisk } from '../../mock-disk';

interface MockedFile {
  absolutePath: string;
  after: JsonFile<PackageJson>;
  before: JsonFile<PackageJson>;
  diskWriteWhenChanged: [string, string];
  id: string;
  logEntryWhenChanged: [any, any];
  logEntryWhenUnchanged: [any, any];
  relativePath: string;
}

export interface TestScenario {
  config: Context['config']['rcFile'];
  disk: MockDisk;
  log: jest.SpyInstance;
  files: Record<string, MockedFile>;
  report: {
    semverGroups: SemverGroupReport[][];
    versionGroups: VersionGroupReport[][];
  };
}

export function createScenario(
  fileMocks: {
    path: string;
    before: JsonFile<PackageJson>;
    after: JsonFile<PackageJson>;
  }[],
  config: Context['config']['rcFile'],
): TestScenario {
  jest.clearAllMocks();
  const disk = mockDisk();
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
      diskWriteWhenChanged: [
        expect.stringContaining(relativePath),
        file.after.json,
      ],
      id: file.path,
      logEntryWhenChanged: [
        expect.stringMatching(/✓/),
        expect.stringContaining(relativePath),
      ],
      logEntryWhenUnchanged: [
        expect.stringMatching(/-/),
        expect.stringContaining(relativePath),
      ],
      relativePath,
    };
  });
  // mock rcfile
  disk.readConfigFileSync.mockImplementation(() => {
    return config;
  });
  // mock file system
  disk.readFileSync.mockImplementation((filePath): string | undefined => {
    return mockedFiles.find((file) => {
      return normalize(filePath) === normalize(file.absolutePath);
    })?.before?.json;
  });
  // mock globs
  disk.globSync.mockImplementation((pattern): string[] => {
    return mockedFiles
      .filter((file) => {
        return minimatch(
          normalize(file.absolutePath),
          toPosix(join(CWD, pattern)),
        );
      })
      .map((file) => normalize(file.absolutePath));
  });
  // create reports
  const ctx = getContext({}, disk);
  const versionGroups = getVersionGroups(ctx);
  const versionGroupsReport = versionGroups.map((group) => group.inspect());
  const semverGroups = getSemverGroups(ctx);
  const semverGroupsReport = semverGroups.map((group) => group.inspect());
  // return API
  return {
    config,
    disk,
    log,
    files: mockedFiles.reduce((memo, file) => {
      memo[file.id] = file;
      return memo;
    }, {} as Record<string, MockedFile>),
    report: {
      semverGroups: semverGroupsReport,
      versionGroups: versionGroupsReport,
    },
  };
}

function toPosix(value: string): string {
  return value.replace('C:', '').replace(/\\/g, '/');
}
