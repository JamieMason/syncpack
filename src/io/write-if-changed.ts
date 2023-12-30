import chalk from 'chalk';
import { Effect, pipe } from 'effect';
import type { Io } from '.';
import { ICON } from '../constants';
import type { Ctx } from '../get-context';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { toJson } from './to-json';
import type { WriteFileError } from './write-file-sync';
import { writeFileSync } from './write-file-sync';

export function writeIfChanged(ctx: Ctx): Effect.Effect<Io, WriteFileError, Ctx> {
  return pipe(
    Effect.all(
      ctx.packageJsonFiles.map((file: PackageJsonFile) =>
        pipe(
          Effect.Do,
          Effect.bind('nextJson', () => Effect.succeed(toJson(ctx, file))),
          Effect.bind('hasChanged', ({ nextJson }) =>
            Effect.succeed(file.jsonFile.json !== nextJson),
          ),
          Effect.flatMap(({ hasChanged, nextJson }) =>
            hasChanged
              ? pipe(
                  writeFileSync(file.jsonFile.filePath, nextJson),
                  Effect.flatMap(() =>
                    Effect.logInfo(chalk`{green ${ICON.tick}} ${file.jsonFile.shortPath}`),
                  ),
                )
              : Effect.logInfo(chalk`{dim ${ICON.skip} ${file.jsonFile.shortPath}}`),
          ),
        ),
      ),
    ),
    Effect.map(() => ctx),
  );
}
