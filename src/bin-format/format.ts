import { Context, Effect, flow, pipe } from 'effect';
import { isArray } from 'tightrope/guard/is-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import { getSortAz } from '../config/get-sort-az';
import { getSortExports } from '../config/get-sort-exports';
import { getSortFirst } from '../config/get-sort-first';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import type { Ctx } from '../get-context';
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
    Effect.flatMap(pipeline),
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

export function pipeline(ctx: Ctx): Effect.Effect<never, never, Ctx> {
  const { config, packageJsonFiles } = ctx;
  const sortAz = getSortAz(config);
  const sortExports = getSortExports(config);
  const sortFirst = getSortFirst(config);
  const sortPackages = config.rcFile.sortPackages !== false;
  const formatBugs = config.rcFile.formatBugs !== false;
  const formatRepository = config.rcFile.formatRepository !== false;

  packageJsonFiles.forEach((file) => {
    const { contents } = file.jsonFile;
    const chain: any = contents;

    if (formatBugs) {
      const bugsUrl = chain?.bugs?.url;
      if (bugsUrl) {
        contents.bugs = bugsUrl;
      }
    }

    if (formatRepository) {
      const repoUrl = chain?.repository?.url;
      const repoDir = chain?.repository?.directory;
      if (isNonEmptyString(repoUrl) && !isNonEmptyString(repoDir)) {
        contents.repository = repoUrl.includes('github.com')
          ? repoUrl.replace(/^.+github\.com\//, '')
          : repoUrl;
      }
    }

    if (sortExports.length > 0) {
      visitExports(sortExports, contents.exports);
    }

    if (sortAz.length > 0) {
      sortAz.forEach((key) => sortAlphabetically(contents[key]));
    }

    if (sortPackages) {
      const sortedKeys = Object.keys(contents).sort();
      sortObject(sortedKeys, contents);
    }

    if (sortFirst.length > 0) {
      const otherKeys = Object.keys(contents);
      const sortedKeys = new Set([...sortFirst, ...otherKeys]);
      sortObject(sortedKeys, contents);
    }
  });

  return Effect.succeed(ctx);
}

function visitExports(sortExports: string[], value: unknown): void {
  if (isObject(value)) {
    const otherKeys = Object.keys(value);
    const sortedKeys = new Set([...sortExports, ...otherKeys]);
    sortObject(sortedKeys, value);
    Object.values(value).forEach((nextValue) => visitExports(sortExports, nextValue));
  }
}

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
