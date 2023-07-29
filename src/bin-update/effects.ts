import * as Data from '@effect/data/Data';
import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import * as Schema from '@effect/schema/Schema';
import chalk from 'chalk';
import https from 'https';
import ora from 'ora';
import type { Choice } from 'prompts';
import prompts from 'prompts';
import { unwrap } from 'tightrope/result/unwrap';
import { ICON } from '../constants';
import type { VersionEffectInput as Input, VersionEffects } from '../create-program/effects';
import type { VersionGroupReport } from '../get-version-groups';
import { getHighestVersion } from '../get-version-groups/lib/get-highest-version';
import { getUniqueSpecifiers } from '../get-version-groups/lib/get-unique-specifiers';
import { logVerbose } from '../lib/log-verbose';
import { getSemverRange, setSemverRange } from '../lib/set-semver-range';

interface InputWithVersions {
  input: Input<VersionGroupReport.Any>;
  versions: {
    all: string[];
    latest: string;
  };
}

type ChoiceAndResult = Choice & { result: InputWithVersions };

let spinner: ora.Ora;
let fetchCount = 0;

export const updateEffects: VersionEffects<InputWithVersions | void> = {
  onFilteredOut() {
    return Effect.unit();
  },
  onIgnored() {
    return Effect.unit();
  },
  onValid(input) {
    return fetchPackageVersions(input);
  },
  onBanned() {
    return Effect.unit();
  },
  onHighestSemverMismatch(input) {
    return fetchPackageVersions(input);
  },
  onLowestSemverMismatch() {
    return Effect.unit();
  },
  onPinnedMismatch() {
    return Effect.unit();
  },
  onSameRangeMismatch(input) {
    return fetchPackageVersions(input);
  },
  onSnappedToMismatch() {
    return Effect.unit();
  },
  onNonSemverMismatch() {
    return Effect.unit();
  },
  onLocalPackageMismatch() {
    return Effect.unit();
  },
  onComplete(ctx, results) {
    return promptForUpdates(results);
  },
};

const safeGetVersion = Schema.parseEither(
  Schema.struct({
    'dist-tags': Schema.struct({ latest: Schema.string }),
    'time': Schema.record(Schema.string, Schema.string),
  }),
);

class HttpError extends Data.TaggedClass('HttpError')<{
  error: string;
}> {}

class JsonParseError extends Data.TaggedClass('JsonParseError')<{
  error: string;
}> {}

function fetchPackageVersions<T extends VersionGroupReport.Any>(input: Input<T>) {
  if (!spinner) spinner = ora().start();
  fetchCount++;
  spinner.text = chalk.blue(`Checked updates for ${fetchCount} dependencies`);
  return pipe(
    fetchUrl(`https://registry.npmjs.org/${input.report.name}`),
    Effect.flatMap(safeGetVersion),
    Effect.map(
      (struct): InputWithVersions => ({
        input,
        versions: {
          all: Object.keys(struct.time).filter((key) => key !== 'modified' && key !== 'created'),
          latest: struct['dist-tags'].latest,
        },
      }),
    ),
    Effect.catchTags({
      HttpError(err) {
        return Effect.sync(() => {
          logVerbose(`HttpError for "${input.report.name}" ${err}`);
        });
      },
      JsonParseError(err) {
        return Effect.sync(() => {
          logVerbose(`JsonParseError for "${input.report.name}" ${err}`);
        });
      },
      ParseError(err) {
        return Effect.sync(() => {
          logVerbose(`ParseError for "${input.report.name}" ${err}`);
        });
      },
    }),
  );
}

function promptForUpdates(results: Array<InputWithVersions | void>) {
  spinner.stop();
  return pipe(
    Effect.Do(),
    Effect.bind('choices', () =>
      Effect.sync(() =>
        results.reduce((arr: ChoiceAndResult[], result): ChoiceAndResult[] => {
          if (!result) return arr;
          if (!['SameRange', 'Standard'].includes(result.input.group._tag)) return arr;

          const input = result.input;
          const latestVersion = result.versions.latest;
          const uniqueVersions = getUniqueSpecifiers(input.report.instances).map(
            (i) => i.specifier,
          );
          const highestVersion = unwrap(getHighestVersion(uniqueVersions));
          const exactHighestVersion = setSemverRange('', highestVersion);

          if (exactHighestVersion === latestVersion) return arr;

          const semverRange = getSemverRange(highestVersion);
          const latestWithRange = setSemverRange(semverRange, latestVersion);

          arr.push({
            result,
            selected: true,
            title: chalk`NAME {red OLD} ARROW {green NEW}`
              .replace('NAME', input.report.name)
              .replace('OLD', uniqueVersions.join(chalk.dim(', ')))
              .replace('ARROW', ICON.rightArrow)
              .replace('NEW', latestWithRange),
          });

          return arr;
        }, []),
      ),
    ),
    Effect.bind('chosenUpdates', ({ choices }) =>
      Effect.tryCatchPromise(
        () =>
          prompts([
            {
              name: 'indexes',
              type: 'multiselect',
              message: 'Choose which packages to upgrade',
              instructions: true,
              // @ts-expect-error optionsPerPage *does* exist https://github.com/terkelg/prompts#options-7
              optionsPerPage: 50,
              choices: choices,
            },
          ])
            .then(({ indexes = [] }: { indexes: number[] }) => ({ choices, indexes }))
            .then(({ choices, indexes }) => indexes.map((i) => choices[i]!.result)),
        () => new Error('Prompt failed'),
      ),
    ),
    Effect.flatMap(({ chosenUpdates }) =>
      Effect.sync(() => {
        chosenUpdates.forEach(({ input, versions }) => {
          input.report.instances.forEach((instance) => {
            const semverRange = getSemverRange(instance.specifier);
            const latestWithRange = setSemverRange(semverRange, versions.latest);
            instance.setSpecifier(latestWithRange);
          });
        });
      }),
    ),
    Effect.catchAll(() => Effect.unit()),
  );
}

// @TODO: add a cache with a short TTL on disk in $TMPDIR
function fetchUrl(url: string): Effect.Effect<never, HttpError | JsonParseError, string> {
  return pipe(
    Effect.async<never, HttpError, string>((resume) => {
      https
        .get(url, (res) => {
          let body = '';
          res.setEncoding('utf8');
          res.on('data', (chunk) => {
            body = `${body}${chunk}`;
          });
          res.on('end', () => {
            resume(Effect.succeed(body));
          });
        })
        .on('error', (err) => {
          resume(Effect.fail(new HttpError({ error: String(err) })));
        });
    }),
    Effect.flatMap((body) =>
      Effect.tryCatch(
        () => JSON.parse(body),
        (err) => new JsonParseError({ error: String(err) }),
      ),
    ),
  );
}
