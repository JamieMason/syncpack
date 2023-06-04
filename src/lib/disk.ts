import { cosmiconfigSync } from 'cosmiconfig';
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore Select *does* exist
import { Input, Select } from 'enquirer';
import {
  readFileSync,
  readJsonSync,
  removeSync,
  writeFileSync,
} from 'fs-extra';
import { sync as globSync } from 'glob';
import { join } from 'path';
import { sync as readYamlSync } from 'read-yaml-file';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import type { O } from 'ts-toolbelt';
import type { RcConfig } from '../config/types';
import { CWD } from '../constants';
import { verbose } from './log';

export type Disk = {
  askForChoice: (opts: {
    message: string;
    choices: string[];
  }) => Promise<string>;
  askForInput: (opts: { message: string }) => Promise<string>;
  globSync: (pattern: string) => string[];
  process: {
    exit: (code: number) => void;
  };
  readConfigFileSync: (configPath?: string) => O.Partial<RcConfig, 'deep'>;
  readFileSync: (filePath: string) => string;
  readYamlFileSync: <T = unknown>(filePath: string) => T;
  removeSync: (filePath: string) => void;
  writeFileSync: (filePath: string, contents: string) => void;
};

const client = cosmiconfigSync('syncpack');

export const disk: Disk = {
  askForChoice({ message, choices }) {
    return new Select({ name: 'choice', message, choices })
      .run()
      .catch(console.error);
  },
  askForInput({ message }) {
    return new Input({ message }).run().catch(console.error);
  },
  globSync(pattern) {
    verbose('globSync(', pattern, ')');
    return globSync(pattern, {
      ignore: '**/node_modules/**',
      absolute: true,
      cwd: CWD,
    });
  },
  process: {
    exit(code) {
      verbose('exit(', code, ')');
      process.exit(code);
    },
  },
  readConfigFileSync(configPath) {
    verbose('readConfigFileSync(', configPath, ')');
    try {
      const result = configPath ? client.load(configPath) : client.search();
      if (result === null) {
        const rcPath = join(CWD, 'package.json');
        const pjson = readJsonSync(rcPath, { throws: false });
        const rcConfig = pjson?.config?.syncpack;
        if (isNonEmptyObject(rcConfig)) return rcConfig;
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
  readFileSync(filePath) {
    verbose('readFileSync(', filePath, ')');
    return readFileSync(filePath, { encoding: 'utf8' });
  },
  readYamlFileSync<T = unknown>(filePath: string): T {
    verbose('readYamlFileSync(', filePath, ')');
    return readYamlSync<T>(filePath);
  },
  removeSync(filePath) {
    verbose('removeSync(', filePath, ')');
    removeSync(filePath);
  },
  writeFileSync(filePath, contents) {
    verbose('writeFileSync(', filePath, contents, ')');
    writeFileSync(filePath, contents);
  },
};
