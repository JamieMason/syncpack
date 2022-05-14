import { cosmiconfigSync } from 'cosmiconfig';
import { readFileSync, removeSync, writeFileSync } from 'fs-extra';
import { sync as globSync } from 'glob';
import { sync as readYamlSync } from 'read-yaml-file';
import type { SyncpackConfig } from '../constants';
import { CWD } from '../constants';
import { verbose } from './log';

export type Disk = typeof disk;

const client = cosmiconfigSync('syncpack');

export const disk = {
  process: {
    exit(code: number): void {
      verbose('exit(', code, ')');
      process.exit(code);
    },
  },
  globSync(pattern: string): string[] {
    verbose('globSync(', pattern, ')');
    return globSync(pattern, {
      ignore: '**/node_modules/**',
      absolute: true,
      cwd: CWD,
    });
  },
  readConfigFileSync(configPath?: string): Partial<SyncpackConfig> {
    verbose('readConfigFileSync(', configPath, ')');
    try {
      const result = configPath ? client.load(configPath) : client.search();
      if (result === null) {
        verbose('no config file found');
        return {};
      }
      const rcPath = result.filepath;
      const rcConfig = result.config;
      verbose('.syncpackrc path:', rcPath);
      verbose('.syncpackrc contents:', rcConfig);
      return rcConfig;
    } catch (err) {
      verbose('no config file found at:', configPath);
      return {};
    }
  },
  readFileSync(filePath: string): string {
    verbose('readFileSync(', filePath, ')');
    return readFileSync(filePath, { encoding: 'utf8' });
  },
  readYamlFileSync<T = unknown>(filePath: string): T {
    verbose('readYamlFileSync(', filePath, ')');
    return readYamlSync<T>(filePath);
  },
  removeSync(filePath: string): void {
    verbose('removeSync(', filePath, ')');
    removeSync(filePath);
  },
  writeFileSync(filePath: string, contents: string): void {
    verbose('writeFileSync(', filePath, contents, ')');
    writeFileSync(filePath, contents);
  },
} as const;
