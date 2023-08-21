import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { EOL } from 'os';
import { getIndent } from '../config/get-indent';
import { ICON } from '../constants';
import type { Ctx } from '../get-context';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { type Env } from './create-env';
import type { WriteFileError } from './tags';
import { EnvTag } from './tags';

export function writeIfChanged(ctx: Ctx): Effect.Effect<Env, WriteFileError, Ctx> {
  return pipe(
    EnvTag,
    Effect.flatMap((env) =>
      Effect.all(
        ctx.packageJsonFiles.map((file: PackageJsonFile) =>
          pipe(
            Effect.Do,
            Effect.bind('nextJson', () => toJson(file)),
            Effect.bind('hasChanged', ({ nextJson }) =>
              Effect.succeed(file.jsonFile.json !== nextJson),
            ),
            Effect.flatMap(({ hasChanged, nextJson }) =>
              hasChanged
                ? pipe(
                    env.writeFileSync(file.jsonFile.filePath, nextJson),
                    Effect.flatMap(() =>
                      Effect.sync(() => {
                        console.log(chalk`{green ${ICON.tick}}`, file.jsonFile.shortPath);
                      }),
                    ),
                  )
                : Effect.sync(() => {
                    console.log(chalk.dim(ICON.skip), chalk.dim(file.jsonFile.shortPath));
                  }),
            ),
          ),
        ),
      ),
    ),
    Effect.map(() => ctx),
  );

  function toJson(file: PackageJsonFile): Effect.Effect<never, never, string> {
    const contents = file.jsonFile.contents;
    const indent = getIndent(ctx.config);
    const EOL = newlines.detect(file.jsonFile.json);
    const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
    return Effect.succeed(newlines.fix(source, EOL));
  }
}

type Ending = '\n' | '\r' | '\r\n' | string;

const CR = '\r';
const CRLF = '\r\n';
const LF = '\n';

export const newlines = {
  detect(source: string): Ending {
    const cr = source.split(CR).length;
    const lf = source.split(LF).length;
    const crlf = source.split(CRLF).length;
    if (cr + lf === 0) return EOL;
    if (crlf === cr && crlf === lf) return CRLF;
    if (cr > lf) return CR;
    return LF;
  },
  fix(source: string, lineEnding: Ending): string {
    return source.replace(/\r\n|\n|\r/g, lineEnding);
  },
};
