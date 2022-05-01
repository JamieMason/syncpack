import { cosmiconfigSync } from 'cosmiconfig';
import { isObject } from 'expect-more';
import { readFileSync, removeSync, writeFileSync } from 'fs-extra';
import { sync as globSync } from 'glob';
import { sync as readYamlSync } from 'read-yaml-file';
import type { SyncpackConfig } from '../constants';
import { CWD } from '../constants';
import { verbose } from './log';

export type Disk = typeof disk;

const client = cosmiconfigSync('syncpack');

export const disk = {
  globSync(pattern: string): string[] {
    return globSync(pattern, {
      ignore: '**/node_modules/**',
      absolute: true,
      cwd: CWD,
    });
  },
  readConfigFileSync(configPath?: string): Partial<SyncpackConfig> {
    try {
      const result = configPath ? client.load(configPath) : client.search();
      const rcConfig: unknown = result !== null ? result.config : {};
      const rcFile = isObject<Partial<SyncpackConfig>>(rcConfig)
        ? rcConfig
        : {};
      return rcFile;
    } catch (err) {
      verbose(`no config file found at ${configPath}`);
      return {};
    }
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
