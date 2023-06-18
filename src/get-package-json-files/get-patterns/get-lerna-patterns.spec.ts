import { match } from '@effect/data/Either';
import { identity, pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import type { MockEnv } from '../../../test/mock-env';
import { createMockEnv } from '../../../test/mock-env';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import { getLernaPatterns } from './get-lerna-patterns';

function runSync(mockedEffects: MockEnv) {
  return pipe(
    Effect.runSyncEither(
      pipe(getLernaPatterns(), Effect.provideService(EnvTag, createEnv(mockedEffects))),
    ),
    match(identity, identity),
  );
}

it('returns strings when found', () => {
  const env = createMockEnv();
  env.readFileSync.mockReturnValue(JSON.stringify({ packages: ['a', 'b'] }));
  expect(runSync(env)).toEqual(O.some(['a', 'b']));
});

it('returns none when a file cannot be read', () => {
  const env = createMockEnv();
  env.readFileSync.mockImplementation(() => {
    throw new Error('Failed to read JSON file');
  });
  expect(runSync(env)).toEqual(O.none());
});

it('returns none when file is not valid JSON', () => {
  const env = createMockEnv();
  env.readFileSync.mockReturnValue('wut?');
  expect(runSync(env)).toEqual(O.none());
});

it('returns none when data is valid JSON but the wrong shape', () => {
  const env = createMockEnv();
  env.readFileSync.mockReturnValue(JSON.stringify({ packages: [1, 2] }));
  expect(runSync(env)).toEqual(O.none());
});
