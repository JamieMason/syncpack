import minimatch from 'minimatch';
import { join, normalize } from 'path';
import { CWD } from '../../src/constants';
import type { Config } from '../../src/lib/get-context/get-config/config';
import type { Source } from '../../src/lib/get-context/get-wrappers';
import type { JsonFile } from '../../src/lib/get-context/get-wrappers/get-patterns/read-json-safe';
import type { MockDisk } from '../mock-disk';
import { mockDisk } from '../mock-disk';

interface MockedFile {
  absolutePath: string;
  after: JsonFile<Source>;
  before: JsonFile<Source>;
  diskWriteWhenChanged: [string, string];
  id: string;
  logEntryWhenChanged: [any, any];
  logEntryWhenUnchanged: [any, any];
  relativePath: string;
}

export interface TestScenario {
  config: Partial<Config.All>;
  disk: MockDisk;
  log: jest.SpyInstance;
  files: Record<string, MockedFile>;
}

export function createScenario(
  fileMocks: {
    path: string;
    before: JsonFile<Source>;
    after: JsonFile<Source>;
  }[],
  config: Partial<Config.All>,
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
