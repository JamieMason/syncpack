import chalk from 'chalk-template';
import { Effect, pipe } from 'effect';
import { ICON } from '../constants.js';
import type { Ctx } from '../get-context/index.js';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file.js';
import type { Io } from './index.js';
import type { WriteFileError } from './write-file-sync.js';
import { writeFileSync } from './write-file-sync.js';

export function writeIfChanged(
  ctx: Ctx,
): Effect.Effect<Ctx, WriteFileError, Io> {
  return pipe(
    Effect.all(
      ctx.packageJsonFiles.map((file: PackageJsonFile) =>
        file.jsonFile.json !== file.nextJson
          ? pipe(
              writeFileSync(file.jsonFile.filePath, file.nextJson),
              Effect.flatMap(() =>
                Effect.logInfo(
                  chalk`{green ${ICON.tick}} ${file.jsonFile.shortPath}`,
                ),
              ),
            )
          : Effect.logInfo(
              chalk`{dim ${ICON.skip} ${file.jsonFile.shortPath}}`,
            ),
      ),
    ),
    Effect.map(() => ctx),
  );
}
