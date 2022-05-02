import minimatch from 'minimatch';
import path from 'path';
import type { SyncpackConfig } from '../../src/constants';
import type { SourceWrapper } from '../../src/lib/get-input/get-wrappers';
import { mockDisk } from '../mock-disk';

interface MockedFile {
  absolutePath: string;
  after: SourceWrapper;
  before: SourceWrapper;
  diskWriteWhenChanged: [string, string];
  logEntryWhenChanged: [any, any];
  logEntryWhenUnchanged: [any, any];
  relativePath: string;
}

export function createScenario(
  fileMocks: {
    path: string;
    before: SourceWrapper;
    after: SourceWrapper;
  }[],
  config: Partial<SyncpackConfig & { configPath: string | undefined }>,
) {
  const disk = mockDisk();
  const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
  // resolve all paths
  const mockedFiles: MockedFile[] = fileMocks.map((file) => ({
    absolutePath: path.join(process.cwd(), file.path),
    after: file.after,
    before: file.before,
    diskWriteWhenChanged: [expect.stringContaining(file.path), file.after.json],
    logEntryWhenChanged: [
      expect.stringMatching(/✓/),
      expect.stringMatching(file.path),
    ],
    logEntryWhenUnchanged: [
      expect.stringMatching(/-/),
      expect.stringMatching(file.path),
    ],
    relativePath: file.path,
  }));
  // mock file system
  disk.readFileSync.mockImplementation((filePath): string | undefined => {
    return mockedFiles.find((file) => {
      return filePath === file.absolutePath;
    })?.before?.json;
  });
  // mock globs
  disk.globSync.mockImplementation((pattern): string[] => {
    return mockedFiles
      .filter((file) => {
        return minimatch(file.absolutePath, path.join(process.cwd(), pattern));
      })
      .map((file) => file.absolutePath);
  });
  // return API
  return {
    config,
    disk,
    log,
    files: mockedFiles.reduce((memo, file) => {
      memo[file.relativePath] = file;
      return memo;
    }, {} as Record<string, MockedFile>),
  };
}
