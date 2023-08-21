import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import { dirname, relative } from 'path';
import type { O } from 'ts-toolbelt';
import type { RcConfig } from '../config/types';
import type { DefaultEnv } from './default-env';
import type { JsonFile } from './tags';
import {
  AskForChoiceError,
  AskForInputError,
  GlobError,
  JsonParseError,
  ReadConfigFileError,
  ReadFileError,
  ReadYamlFileError,
  WriteFileError,
} from './tags';

export interface Env {
  readonly askForChoice: (opts: {
    message: string;
    choices: string[];
  }) => Effect.Effect<never, AskForChoiceError, string>;
  readonly askForInput: (opts: {
    message: string;
  }) => Effect.Effect<never, AskForInputError, string>;
  readonly CWD: string;
  readonly exitProcess: (code: number) => Effect.Effect<never, never, void>;
  readonly globSync: (patterns: string[]) => Effect.Effect<never, GlobError, string[]>;
  readonly readConfigFileSync: (
    configPath?: string,
  ) => Effect.Effect<never, ReadConfigFileError, O.Partial<RcConfig, 'deep'>>;
  readonly readFileSync: (filePath: string) => Effect.Effect<never, ReadFileError, string>;
  readonly readJsonFileSync: <T>(
    filePath: string,
  ) => Effect.Effect<never, ReadFileError | JsonParseError, JsonFile<T>>;
  readonly readYamlFileSync: <T = unknown>(
    filePath: string,
  ) => Effect.Effect<never, ReadYamlFileError, T>;
  readonly writeFileSync: (
    filePath: string,
    contents: string,
  ) => Effect.Effect<never, WriteFileError, void>;
}

export function createEnv(env: DefaultEnv): Env {
  return {
    askForChoice(opts) {
      return Effect.tryPromise({
        try: () => env.askForChoice(opts),
        catch: (err) => new AskForChoiceError({ error: String(err) }),
      });
    },
    askForInput(opts) {
      return Effect.tryPromise({
        try: () => env.askForInput(opts),
        catch: (err) => new AskForInputError({ error: String(err) }),
      });
    },
    // @FIXME: process.exit is probably handled some other way in effect-ts
    exitProcess(code) {
      return Effect.sync(() => {
        env.exitProcess(code);
      });
    },
    CWD: env.CWD,
    globSync(patterns) {
      return Effect.try({
        try: () => env.globSync(patterns),
        catch: (err) => new GlobError({ error: String(err) }),
      });
    },
    readConfigFileSync(filePath) {
      return Effect.try({
        try: () => env.readConfigFileSync(filePath),
        catch: (err) => new ReadConfigFileError({ filePath: filePath || '', error: String(err) }),
      });
    },
    readFileSync(filePath) {
      return Effect.try({
        try: () => env.readFileSync(filePath),
        catch: (err) => new ReadFileError({ filePath, error: String(err) }),
      });
    },
    readJsonFileSync(filePath) {
      return pipe(
        Effect.Do,
        Effect.bind('json', () =>
          Effect.try({
            try: () => env.readFileSync(filePath),
            catch: (err) => new ReadFileError({ filePath, error: String(err) }),
          }),
        ),
        Effect.bind('contents', ({ json }) =>
          Effect.try({
            try: () => JSON.parse(json),
            catch: (err) => new JsonParseError({ error: String(err), filePath, json }),
          }),
        ),
        Effect.map(({ contents, json }) => ({
          contents,
          dirPath: dirname(filePath),
          filePath,
          json,
          shortPath: relative(env.CWD, filePath),
        })),
      );
    },
    readYamlFileSync(filePath) {
      return Effect.try({
        try: () => env.readYamlFileSync(filePath),
        catch: (err) => new ReadYamlFileError({ filePath, error: String(err) }),
      });
    },
    writeFileSync(filePath, contents) {
      return Effect.try({
        try: () => env.writeFileSync(filePath, contents),
        catch: (err) => new WriteFileError({ filePath, error: String(err) }),
      });
    },
  };
}
