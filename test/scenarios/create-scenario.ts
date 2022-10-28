import minimatch from 'minimatch';
import { join, normalize } from 'path';
import type { SyncpackConfig } from '../../src/types';
import { CWD } from '../../src/constants';
import type { SourceWrapper } from '../../src/lib/get-input/get-wrappers';
import type { MockDisk } from '../mock-disk';
import { mockDisk } from '../mock-disk';

interface MockedFile {
  absolutePath: string;
  after: SourceWrapper;
  before: SourceWrapper;
  diskWriteWhenChanged: [string, string];
  id: string;
  logEntryWhenChanged: [any, any];
  logEntryWhenUnchanged: [any, any];
  relativePath: string;
}

export interface TestScenario {
  config: Partial<SyncpackConfig & { configPath: string | undefined }>;
  disk: MockDisk;
  log: jest.SpyInstance;
  files: Record<string, MockedFile>;
}

export function createScenario(
  fileMocks: {
    path: string;
    before: SourceWrapper;
    after: SourceWrapper;
  }[],
  config: Partial<SyncpackConfig & { configPath: string | undefined }>,
): TestScenario {
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
  // return API
  return {
    config,
    disk,
    log,
    files: mockedFiles.reduce((memo, file) => {
      memo[file.id] = file;
      return memo;
    }, {} as Record<string, MockedFile>),
  };
}

function toPosix(value: string): string {
  return value.replace('C:', '').replace(/\\/g, '/');
}
