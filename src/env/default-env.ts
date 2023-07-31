import { cosmiconfigSync } from 'cosmiconfig';
import { pipe } from 'tightrope/fn/pipe';
import { fromTry } from 'tightrope/result/from-try';
import { unwrapOrElse } from 'tightrope/result/unwrap-or-else';
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore Select *does* exist
import { Input, Select } from 'enquirer';
import { readFileSync, writeFileSync } from 'fs';
import * as globby from 'globby';
import { join } from 'path';
import * as readYamlFile from 'read-yaml-file';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import { Ok } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { map } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import type { O } from 'ts-toolbelt';
import type { RcConfig } from '../config/types';
import { CWD } from '../constants';
import { logVerbose } from '../lib/log-verbose';

export interface DefaultEnv {
  readonly askForChoice: (opts: { message: string; choices: string[] }) => Promise<string>;
  readonly askForInput: (opts: { message: string }) => Promise<string>;
  readonly exitProcess: (code: number) => void;
  readonly globSync: (patterns: string[]) => string[];
  readonly readConfigFileSync: (configPath?: string) => O.Partial<RcConfig, 'deep'>;
  readonly readFileSync: (filePath: string) => string;
  readonly readYamlFileSync: <T = unknown>(filePath: string) => T;
  readonly writeFileSync: (filePath: string, contents: string) => void;
}

export const defaultEnv: DefaultEnv = {
  /* istanbul ignore next */
  askForChoice({ message, choices }) {
    return new Select({ name: 'choice', message, choices }).run();
  },
  /* istanbul ignore next */
  askForInput({ message }) {
    return new Input({ message }).run();
  },
  /* istanbul ignore next */
  globSync(patterns) {
    logVerbose('globSync(', patterns, ')');
    return globby.sync(patterns, {
      ignore: ['**/node_modules/**'],
      absolute: true,
      cwd: CWD,
    });
  },
  /* istanbul ignore next */
  exitProcess(code) {
    logVerbose('exit(', code, ')');
    process.exit(code);
  },
  readConfigFileSync(configPath) {
    logVerbose('readConfigFileSync(', configPath, ')');
    const client = cosmiconfigSync('syncpack');
    const result = configPath ? client.load(configPath) : client.search();
    if (!isNonEmptyObject(result)) {
      const rcPath = join(CWD, 'package.json');
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
  /* istanbul ignore next */
  readFileSync(filePath) {
    logVerbose('readFileSync(', filePath, ')');
    return readFileSync(filePath, { encoding: 'utf8' });
  },
  /* istanbul ignore next */
  readYamlFileSync<T = unknown>(filePath: string): T {
    logVerbose('readYamlFileSync(', filePath, ')');
    return readYamlFile.sync<T>(filePath);
  },
  /* istanbul ignore next */
  writeFileSync(filePath, contents) {
    logVerbose('writeFileSync(', filePath, contents, ')');
    writeFileSync(filePath, contents);
  },
};
