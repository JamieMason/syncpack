import { match } from '@effect/data/Either';
import { identity, pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import 'expect-more-jest';
import type { MockEnv } from '../../test/mock-env';
import { createMockEnv } from '../../test/mock-env';
import { CWD } from '../constants';
import { createEnv } from '../env/create-env';
import { EnvTag } from '../env/tags';
import type { Ctx } from '../get-context';
import { getFilePaths, NoSourcesFoundError } from './get-file-paths';

function runSync(config: Ctx['config'], mockedEffects: MockEnv) {
  return pipe(
    Effect.runSyncEither(
      pipe(getFilePaths(config), Effect.provideService(EnvTag, createEnv(mockedEffects))),
    ),
    match(identity, identity),
  );
}

it('return error when patterns return no files', () => {
  const env = createMockEnv();
  env.globSync.mockReturnValue([]);
  const result = runSync({ cli: {}, rcFile: {} }, env);
  expect(result).toEqual(
    new NoSourcesFoundError({
      CWD,
      patterns: ['package.json', 'packages/*/package.json'],
    }),
  );
});

it('returns strings when patterns return files', () => {
  const env = createMockEnv();
  const root = ['/fake/dir/package.json'];
  const packages = ['/fake/dir/packages/a/package.json', '/fake/dir/packages/b/package.json'];
  env.globSync.mockImplementation(() => {
    return [...root, ...packages];
  });
  const result = runSync({ cli: {}, rcFile: {} }, env);
  expect(result).toEqual([...root, ...packages]);
});
