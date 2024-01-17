import * as Schema from '@effect/schema/Schema';
import chalk from 'chalk-template';
import { Data, Effect, identity, pipe } from 'effect';
import https from 'https';
import ora, { type Ora } from 'ora';
import { EOL } from 'os';
import prompts from 'prompts';
import type { ReleaseType } from 'semver';
import { diff } from 'semver';
import gtr from 'semver/ranges/gtr.js';
import { isArray } from 'tightrope/guard/is-array.js';
import { isEmptyObject } from 'tightrope/guard/is-empty-object.js';
import { ICON } from '../constants.js';
import type { Instance } from '../get-instances/instance.js';
import { formatRepositoryUrl } from '../lib/format-repository-url.js';
import { RingBuffer } from '../lib/ring-buffer.js';
import { setSemverRange } from '../lib/set-semver-range.js';
import { Specifier } from '../specifier/index.js';

type ReleasesByType = Record<ReleaseType, Releases[]>;

/** full release history from the npm registry for a given package */
class Releases extends Data.TaggedClass('Releases')<{
  instance: Instance;
  versions: {
    all: string[];
    latest: string;
  };
  repoUrl: string | undefined;
}> {}

// https://github.com/terkelg/prompts?tab=readme-ov-file#prompts
class PromptCancelled extends Data.TaggedClass('PromptCancelled')<{
  name: string;
}> {}

class HttpError extends Data.TaggedClass('HttpError')<{
  error: string;
}> {}

class NpmRegistryError extends Data.TaggedClass('NpmRegistryError')<{
  error: string;
}> {}

/** the API client for the terminal spinner */
let spinner: Ora | null = null;

/** how many HTTP requests have been sent */
let fetchedCount = 0;

/** how many instances have updates available */
let outdatedCount = 0;

/** names of instances currently being fetched from npm */
const inFlight = new Set<string>();

/** names of instances most recently finished being fetched from npm */
const mostRecent = new RingBuffer<string>(5);

/** page size when prompting */
const optionsPerPage = 50;

/** instance names in `inFlight` are formatted for display */
function format(instance: Instance) {
  return chalk`{gray ${instance.name}}`;
}

/** we need to remove colours when sorting loading status output */
function stripAnsi(str: string) {
  // eslint-disable-next-line no-control-regex
  const ansiChars = /[\u001b\u009b][[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]/g;
  return str.replace(ansiChars, '');
}

export const updateEffects = {
  onFetchAllStart() {
    if (!spinner) spinner = ora().start();
    fetchedCount = 0;
    return Effect.unit;
  },
  onFetchStart(instance: Instance, totalCount: number) {
    inFlight.add(format(instance));
    fetchedCount++;
    if (spinner) {
      const indent = `${EOL}  `;
      const progress = new Set([...mostRecent.filter(Boolean), ...inFlight.values()]);
      const sortedProgress = Array.from(progress).sort((a, b) =>
        stripAnsi(a).localeCompare(stripAnsi(b)),
      );
      const suffixText = sortedProgress.join(indent);
      spinner.text = chalk`${outdatedCount} updates found in ${fetchedCount}/${totalCount} dependencies${indent}${suffixText}`;
    }
    return Effect.unit;
  },
  onFetchEnd(instance: Instance, versions?: Releases['versions']) {
    inFlight.delete(format(instance));
    const latest = versions?.latest;
    if (latest) {
      if (gtr(latest, String(instance.rawSpecifier.raw), true)) {
        outdatedCount++;
        mostRecent.push(
          chalk`${instance.name} {gray {red ${instance.rawSpecifier.raw}} ${ICON.rightArrow}} {green ${latest}}`,
        );
      } else {
        mostRecent.push(chalk`{green ${instance.name}}`);
      }
    }
    return Effect.unit;
  },
  /** After checking the registry, store this instance known to be up to date */
  onUpToDate(instance: Instance) {
    mostRecent.push(chalk`{green ${instance.name}}`);
    return Effect.unit;
  },
  /** After checking the registry, store this instance known to have newer versions available */
  onOutdated(instance: Instance, latest: string) {
    outdatedCount++;
    mostRecent.push(
      chalk`${instance.name} {gray {red ${instance.rawSpecifier.raw}} ${ICON.rightArrow}} {green ${latest}}`,
    );
    return Effect.unit;
  },
  /** As the last request completes, remove the progress information */
  onFetchAllEnd() {
    if (spinner) spinner.stop();
    spinner = null;
    fetchedCount = 0;
    return Effect.unit;
  },
  /** Fetch available versions for a given package from the npm registry */
  fetchLatestVersions(
    instance: Instance,
  ): Effect.Effect<never, HttpError | NpmRegistryError, Releases> {
    return pipe(
      fetchJson(`https://registry.npmjs.org/${instance.name}`),
      // parse and validate the specific data we expect
      Effect.flatMap(
        Schema.parse(
          Schema.struct({
            'dist-tags': Schema.struct({ latest: Schema.string }),
            'time': Schema.record(Schema.string, Schema.string),
            'homepage': Schema.optional(Schema.string),
            'repository': Schema.optional(
              Schema.union(Schema.string, Schema.struct({ url: Schema.optional(Schema.string) })),
            ),
          }),
        ),
      ),
      // transform it into something more appropriate
      Effect.map((struct) => {
        const rawRepoUrl =
          typeof struct.repository === 'object' ? struct.repository.url : struct.repository;

        return new Releases({
          instance,
          versions: {
            all: Object.keys(struct.time).filter((key) => key !== 'modified' && key !== 'created'),
            latest: struct['dist-tags'].latest,
          },
          repoUrl: formatRepositoryUrl(rawRepoUrl),
        });
      }),
      // hide ParseErrors and just treat them as another kind of NpmRegistryError
      Effect.catchTags({
        ParseError: () =>
          Effect.fail(new NpmRegistryError({ error: `Invalid response for ${instance.name}` })),
      }),
    );
  },
  /** Given responses from npm, ask the user which they want */
  promptForUpdates(outdated: Releases[]): Effect.Effect<never, PromptCancelled, void> {
    return pipe(
      Effect.Do,
      Effect.bind('releasesByType', () => groupByReleaseType(outdated)),
      // Create choices to ask if they want major, minor, patch etc
      Effect.bind('releaseTypeQuestions', ({ releasesByType }) =>
        Effect.succeed(
          Object.keys(releasesByType)
            .filter((type) => releasesByType[type as ReleaseType].length > 0)
            .map((type) => ({
              title: chalk`${releasesByType[type as ReleaseType].length} ${type}`,
              selected: true,
              value: type,
            })),
        ),
      ),
      // Ask which release types (major, minor, patch etc) they want
      Effect.bind('releaseTypeAnswers', ({ releaseTypeQuestions }) =>
        releaseTypeQuestions.length > 0
          ? pipe(
              Effect.tryPromise({
                try: (): Promise<string[]> =>
                  prompts({
                    name: 'releaseTypeAnswers',
                    type: 'multiselect',
                    instructions: true,
                    message: `${outdated.length} updates are available`,
                    choices: releaseTypeQuestions,
                  }).then((res) => res?.releaseTypeAnswers || []),
                catch: identity,
              }),
              Effect.catchAll(() =>
                pipe(
                  Effect.logError('Error when prompting for releaseTypeAnswers'),
                  Effect.map(() => []),
                ),
              ),
            )
          : Effect.succeed([]),
      ),
      // For each chosen release type, list the available updates to choose from
      Effect.bind('prepatchAnswers', (doState) => promptForReleaseType('prepatch', doState)),
      Effect.bind('patchAnswers', (doState) => promptForReleaseType('patch', doState)),
      Effect.bind('preminorAnswers', (doState) => promptForReleaseType('preminor', doState)),
      Effect.bind('minorAnswers', (doState) => promptForReleaseType('minor', doState)),
      Effect.bind('premajorAnswers', (doState) => promptForReleaseType('premajor', doState)),
      Effect.bind('majorAnswers', (doState) => promptForReleaseType('major', doState)),
      Effect.bind('prereleaseAnswers', (doState) => promptForReleaseType('prerelease', doState)),
      /** Apply every update to the package.json files */
      Effect.flatMap((doState) =>
        pipe(
          [
            ...doState.prepatchAnswers,
            ...doState.patchAnswers,
            ...doState.preminorAnswers,
            ...doState.minorAnswers,
            ...doState.premajorAnswers,
            ...doState.majorAnswers,
            ...doState.prereleaseAnswers,
          ],
          Effect.forEach((release) =>
            pipe(
              release.instance.versionGroup.instances,
              Effect.forEach((instance) =>
                pipe(
                  instance.semverGroup.getFixed(
                    Specifier.create(instance, release.versions.latest),
                  ),
                  Effect.flatMap((latestWithRange) => instance.write(latestWithRange.raw)),
                  Effect.catchTag('NonSemverError', Effect.logError),
                ),
              ),
            ),
          ),
          Effect.flatMap(() => Effect.unit),
        ),
      ),
    );
  },
};

function promptForReleaseType(
  releaseType: ReleaseType,
  doState: { releasesByType: ReleasesByType; releaseTypeAnswers: string[] },
): Effect.Effect<never, PromptCancelled, Releases[]> {
  const { releasesByType, releaseTypeAnswers } = doState;
  const prop = `${releaseType}Answers`;
  const releases = releasesByType[releaseType];
  return releaseTypeAnswers.includes(releaseType)
    ? pipe(
        Effect.tryPromise({
          try: (): Promise<Record<string, unknown>> =>
            prompts({
              name: prop,
              type: 'multiselect',
              instructions: false,
              // @ts-expect-error optionsPerPage *does* exist https://github.com/terkelg/prompts#options-7
              optionsPerPage,
              message: `${releases.length} ${releaseType} updates`,
              choices: releases.map((updateable) => {
                const spacingValue =
                  50 -
                  updateable.instance.name.length -
                  String(updateable.instance.rawSpecifier).length -
                  updateable.versions.latest.length;
                const spacing = Array.from({ length: spacingValue }).fill(' ').join('');

                const repoUrl = updateable.repoUrl
                  ? chalk`${spacing} {white - ${updateable.repoUrl}}`
                  : '';

                return {
                  title: chalk`${updateable.instance.name} {gray ${updateable.instance.rawSpecifier.raw} ${ICON.rightArrow}} {green ${updateable.versions.latest}} ${repoUrl}`,
                  selected: true,
                  value: updateable,
                };
              }),
            }),
          catch: identity,
        }),
        // Paper over errors in terkelg/prompts for now
        Effect.catchAll(() =>
          pipe(
            Effect.logError(`terkelg/prompts errored while prompting for ${prop}`),
            Effect.map(() => ({ [prop]: [] })),
          ),
        ),
        // In terkelg/prompts, an empty object means that the user cancelled via
        // ctrl+c or the escape key etc. Handle this case so we can skip any
        // remaining steps.
        Effect.flatMap((res) =>
          isEmptyObject(res)
            ? Effect.fail(new PromptCancelled({ name: releaseType }))
            : Effect.succeed(isArray(res?.[prop]) ? res?.[prop] : []),
        ),
      )
    : Effect.succeed([]);
}

function groupByReleaseType(releases: Releases[]): Effect.Effect<never, never, ReleasesByType> {
  return Effect.succeed(
    releases.reduce(
      (releasesByType: ReleasesByType, release) => {
        const previous = setSemverRange('', String(release.instance.rawSpecifier.raw));
        const latest = release.versions.latest;
        try {
          const type = diff(previous, latest);
          if (type && releasesByType[type]) {
            releasesByType[type].push(release);
          }
        } catch {
          //
        }
        return releasesByType;
      },
      {
        prepatch: [],
        patch: [],
        preminor: [],
        minor: [],
        premajor: [],
        major: [],
        prerelease: [],
      },
    ),
  );
}

// @TODO: add a cache with a short TTL on disk in $TMPDIR
function fetchJson(url: string): Effect.Effect<never, HttpError | NpmRegistryError, unknown> {
  return pipe(
    Effect.async<never, HttpError, string>((resume) => {
      // setTimeout(
      //   () => {
      //     resume(
      //       Effect.succeed(
      //         JSON.stringify({
      //           'dist-tags': { latest: '3.1.1' },
      //           'time': {
      //             '0.3.1': new Date().toJSON(),
      //           },
      //         }),
      //       ),
      //     );
      //   },
      //   Math.floor(Math.random() * 500) + 1,
      // );
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
          resume(
            Effect.fail(new HttpError({ error: `Node https threw on ${url}: ${String(err)}` })),
          );
        });
    }),
    Effect.flatMap((body) =>
      Effect.try({
        try: () => JSON.parse(body),
        catch: () => new NpmRegistryError({ error: `JSON.parse threw on response from ${url}` }),
      }),
    ),
  );
}
