import { cosmiconfigSync } from 'cosmiconfig';
import { isObject } from 'expect-more';
import { readFileSync, removeSync, writeFileSync } from 'fs-extra';
import { sync as globSync } from 'glob';
import { sync as readYamlSync } from 'read-yaml-file';
import type { SyncpackConfig } from '../constants';
import { CWD } from '../constants';

export type Disk = typeof disk;

export const disk = {
  globSync(pattern: string): string[] {
    return globSync(pattern, {
      ignore: '**/node_modules/**',
      absolute: true,
      cwd: CWD,
    });
  },
  readConfigFileSync(): Partial<SyncpackConfig> {
    const rcSearch = cosmiconfigSync('syncpack').search();
    const rcConfig: unknown = rcSearch !== null ? rcSearch.config : {};
    const rcFile = isObject<Partial<SyncpackConfig>>(rcConfig) ? rcConfig : {};
    return rcFile;
  },
  readFileSync(filePath: string): string {
    return readFileSync(filePath, { encoding: 'utf8' });
  },
  readYamlFileSync<T = unknown>(filePath: string): T {
    return readYamlSync<T>(filePath);
  },
  removeSync(filePath: string): void {
    removeSync(filePath);
  },
  writeFileSync(filePath: string, contents: string): void {
    writeFileSync(filePath, contents);
  },
} as const;
