import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { O } from 'ts-toolbelt';
import { CliConfigTag } from '../config/tag';
import { type CliConfig, type RcConfig } from '../config/types';
import type { Env } from '../env/create-env';
import type { GlobError, JsonParseError, ReadConfigFileError, ReadFileError } from '../env/tags';
import { EnvTag } from '../env/tags';
import { getPackageJsonFiles } from '../get-package-json-files';
import type { NoSourcesFoundError } from '../get-package-json-files/get-file-paths';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';

export interface Ctx {
  readonly config: {
    readonly cli: Partial<CliConfig>;
    readonly rcFile: O.Partial<RcConfig, 'deep'>;
  };
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
}

export function getContext(): Effect.Effect<
  Partial<CliConfig> | Env,
  NoSourcesFoundError | GlobError | ReadFileError | JsonParseError | ReadConfigFileError,
  Ctx
> {
  return pipe(
    Effect.Do,
    Effect.bind('cli', () => CliConfigTag),
    Effect.bind('env', () => EnvTag),
    Effect.bind('rcFile', ({ cli, env }) => env.readConfigFileSync(cli.configPath)),
    Effect.bind('packageJsonFiles', getPackageJsonFiles),
    Effect.map(({ cli, rcFile, packageJsonFiles }) => ({
      config: { cli, rcFile },
      isInvalid: false,
      packageJsonFiles,
    })),
  );
}
