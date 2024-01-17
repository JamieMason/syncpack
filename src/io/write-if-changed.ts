import chalk from 'chalk-template';
import { Effect, pipe } from 'effect';
import { ICON } from '../constants.js';
import type { Ctx } from '../get-context/index.js';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file.js';
import type { Io } from './index.js';
import { toJson } from './to-json.js';
import type { WriteFileError } from './write-file-sync.js';
import { writeFileSync } from './write-file-sync.js';

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
