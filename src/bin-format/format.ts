import { Context, Effect, flow, pipe } from 'effect';
import { isArray } from 'tightrope/guard/is-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import { getSortAz } from '../config/get-sort-az';
import { getSortFirst } from '../config/get-sort-first';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import type { Io } from '../io';
import { IoTag } from '../io';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { writeIfChanged } from '../io/write-if-changed';
import { withLogger } from '../lib/with-logger';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function format({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.map((ctx) => {
      const { packageJsonFiles } = ctx;
      const sortAz = getSortAz(ctx.config);
      const sortFirst = getSortFirst(ctx.config);

      packageJsonFiles.forEach((file) => {
        const { contents } = file.jsonFile;
        const sortedKeys = Object.keys(contents).sort();
        const keys = new Set<string>(sortFirst.concat(sortedKeys));

        const optionalChaining: any = contents;
        const bugsUrl = optionalChaining?.bugs?.url;
        const repoUrl = optionalChaining?.repository?.url;
        const repoDir = optionalChaining?.repository?.directory;

        if (bugsUrl) {
          contents.bugs = bugsUrl;
        }

        if (isNonEmptyString(repoUrl) && !isNonEmptyString(repoDir)) {
          contents.repository = repoUrl.includes('github.com')
            ? repoUrl.replace(/^.+github\.com\//, '')
            : repoUrl;
        }

        sortAz.forEach((key) => sortAlphabetically(contents[key]));
        sortObject(keys, contents);
      });

      return ctx;

      function sortObject(sortedKeys: string[] | Set<string>, obj: Record<string, unknown>): void {
        sortedKeys.forEach((key: string) => {
          const value = obj[key];
          delete obj[key];
          obj[key] = value;
        });
      }

      function sortAlphabetically(value: unknown): void {
        if (isArray(value)) {
          value.sort();
        } else if (isObject(value)) {
          sortObject(Object.keys(value).sort(), value);
        }
      }
    }),
    Effect.flatMap((ctx) =>
      pipe(
        writeIfChanged(ctx),
        Effect.catchTags({
          WriteFileError: flow(
            errorHandlers.WriteFileError,
            Effect.map(() => {
              ctx.isInvalid = true;
              return ctx;
            }),
          ),
        }),
      ),
    ),
    Effect.flatMap(exitIfInvalid),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}
