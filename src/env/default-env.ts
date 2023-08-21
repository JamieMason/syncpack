import { cosmiconfigSync } from 'cosmiconfig';
import { prompt } from 'enquirer';
import { readFileSync, writeFileSync } from 'fs';
import * as globby from 'globby';
import { join } from 'path';
import * as readYamlFile from 'read-yaml-file';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import { Ok } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import { unwrapOrElse } from 'tightrope/result/unwrap-or-else';
import type { O } from 'ts-toolbelt';
import type { RcConfig } from '../config/types';
import { CWD } from '../constants';
import { logVerbose } from '../lib/log-verbose';

export interface DefaultEnv {
  readonly askForChoice: (opts: { message: string; choices: string[] }) => Promise<string>;
  readonly askForInput: (opts: { message: string }) => Promise<string>;
  readonly CWD: string;
  readonly exitProcess: (code: number) => void;
  readonly globSync: (patterns: string[]) => string[];
  readonly readConfigFileSync: (configPath?: string) => O.Partial<RcConfig, 'deep'>;
  readonly readFileSync: (filePath: string) => string;
  readonly readYamlFileSync: <T = unknown>(filePath: string) => T;
  readonly writeFileSync: (filePath: string, contents: string) => void;
}

export const defaultEnv: DefaultEnv = {
  askForChoice({ message, choices }) {
    return prompt({
      type: 'select',
      name: 'choice',
      message,
      choices,
    });
  },
  askForInput({ message }) {
    return prompt({
      name: 'version',
      type: 'input',
      message,
    });
  },
  CWD,
  exitProcess(code) {
    logVerbose('exit(', code, ')');
    process.exit(code);
  },
  globSync(patterns) {
    logVerbose('globSync(', patterns, ')');
    return globby.sync(patterns, {
      ignore: ['**/node_modules/**'],
      absolute: true,
      cwd: defaultEnv.CWD,
    });
  },
  readConfigFileSync(configPath) {
    logVerbose('readConfigFileSync(', configPath, ')');
    const client = cosmiconfigSync('syncpack');
    const result = configPath ? client.load(configPath) : client.search();
    if (!isNonEmptyObject(result)) {
      const rcPath = join(defaultEnv.CWD, 'package.json');
      return pipe(
        fromTry(() => readFileSync(rcPath, { encoding: 'utf8' })),
        map(JSON.parse),
        map((pjson) => pjson?.config?.syncpack),
        filter(isNonEmptyObject, 'no config file found'),
        mapErr((err) => {
          logVerbose('no config file found');
          return err;
        }),
        unwrapOrElse(() => new Ok({})),
      );
    }
    const rcPath = result.filepath;
    const rcConfig = result.config;
    logVerbose('.syncpackrc path:', rcPath);
    logVerbose('.syncpackrc contents:', rcConfig);
    return rcConfig;
  },
  readFileSync(filePath) {
    logVerbose('readFileSync(', filePath, ')');
    return readFileSync(filePath, { encoding: 'utf8' });
  },
  readYamlFileSync<T = unknown>(filePath: string): T {
    logVerbose('readYamlFileSync(', filePath, ')');
    return readYamlFile.sync<T>(filePath);
  },
  writeFileSync(filePath, contents) {
    logVerbose('writeFileSync(', filePath, contents, ')');
    writeFileSync(filePath, contents);
  },
};
