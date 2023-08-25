import { Effect, flow, pipe } from 'effect';
import type { O } from 'ts-toolbelt';
import { type CliConfig, type RcConfig } from '../config/types';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { getPackageJsonFiles } from '../get-package-json-files';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Io } from '../io';
import { readConfigFileSync } from '../io/read-config-file-sync';
import { keyBy } from './lib/key-by';

export interface Ctx {
  readonly config: {
    readonly cli: Partial<CliConfig>;
    readonly rcFile: O.Partial<RcConfig, 'deep'>;
  };
  /** @TODO: Replace with Effect Exit/Cause */
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
  packageJsonFilesByName: Record<string, PackageJsonFile>;
}

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers: ErrorHandlers;
}

export function getContext({ io, cli, errorHandlers }: Input): Effect.Effect<never, never, Ctx> {
  const exitOnError = Effect.flatMap(() => Effect.failSync(() => io.process.exit(1)));
  return pipe(
    Effect.Do,
    Effect.bind('rcFile', () => readConfigFileSync(io, cli.configPath)),
    Effect.bind('packageJsonFiles', ({ rcFile }) => getPackageJsonFiles(io, { cli, rcFile })),
    Effect.map(({ rcFile, packageJsonFiles }) => ({
      config: { cli, rcFile },
      isInvalid: false,
      packageJsonFiles,
      packageJsonFilesByName: keyBy('name', packageJsonFiles),
    })),
    Effect.catchTags({
      GlobError: flow(errorHandlers.GlobError, exitOnError),
      JsonParseError: flow(errorHandlers.JsonParseError, exitOnError),
      NoSourcesFoundError: flow(errorHandlers.NoSourcesFoundError, exitOnError),
      ReadFileError: flow(errorHandlers.ReadFileError, exitOnError),
    }),
  );
}
