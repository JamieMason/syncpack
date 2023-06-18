import * as Effect from '@effect/io/Effect';
import type { O } from 'ts-toolbelt';
import type { RcConfig } from '../config/types';
import type { DefaultEnv } from './default-env';
import {
  AskForChoiceError,
  AskForInputError,
  GlobError,
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
  readonly exitProcess: (code: number) => Effect.Effect<never, never, void>;
  readonly globSync: (patterns: string[]) => Effect.Effect<never, GlobError, string[]>;
  readonly readConfigFileSync: (
    configPath?: string,
  ) => Effect.Effect<never, ReadConfigFileError, O.Partial<RcConfig, 'deep'>>;
  readonly readFileSync: (filePath: string) => Effect.Effect<never, ReadFileError, string>;
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
      return Effect.tryCatchPromise(
        () => env.askForChoice(opts),
        (err) => new AskForChoiceError({ error: String(err) }),
      );
    },
    askForInput(opts) {
      return Effect.tryCatchPromise(
        () => env.askForInput(opts),
        (err) => new AskForInputError({ error: String(err) }),
      );
    },
    // @FIXME: process.exit is probably handled some other way in effect-ts
    exitProcess(code) {
      return Effect.sync(() => {
        env.exitProcess(code);
      });
    },
    globSync(patterns) {
      return Effect.tryCatch(
        () => env.globSync(patterns),
        (err) => new GlobError({ error: String(err) }),
      );
    },
    readConfigFileSync(filePath) {
      return Effect.tryCatch(
        () => env.readConfigFileSync(filePath),
        (err) => new ReadConfigFileError({ filePath: filePath || '', error: String(err) }),
      );
    },
    readFileSync(filePath) {
      return Effect.tryCatch(
        () => env.readFileSync(filePath),
        (err) => new ReadFileError({ filePath, error: String(err) }),
      );
    },
    readYamlFileSync(filePath) {
      return Effect.tryCatch(
        () => env.readYamlFileSync(filePath),
        (err) => new ReadYamlFileError({ filePath, error: String(err) }),
      );
    },
    writeFileSync(filePath, contents) {
      return Effect.tryCatch(
        () => env.writeFileSync(filePath, contents),
        (err) => new WriteFileError({ filePath, error: String(err) }),
      );
    },
  };
}
