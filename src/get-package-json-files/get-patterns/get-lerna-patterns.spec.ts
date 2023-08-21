import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import type { MockEnv } from '../../../test/lib/mock-env';
import { createMockEnv } from '../../../test/lib/mock-env';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import { getLernaPatterns } from './get-lerna-patterns';

function runSync(mockedEffects: MockEnv, onValue: (value: any) => void) {
  Effect.runSync(
    pipe(
      getLernaPatterns(),
      Effect.match({
        onFailure: onValue,
        onSuccess: onValue,
      }),
      Effect.provideService(EnvTag, createEnv(mockedEffects)),
    ),
  );
}

it('returns strings when found', () => {
  const env = createMockEnv();
  env.readFileSync.mockReturnValue(JSON.stringify({ packages: ['a', 'b'] }));
  runSync(env, (value) => {
    expect(value).toEqual(O.some(['a', 'b']));
  });
});

it('returns none when a file cannot be read', () => {
  const env = createMockEnv();
  env.readFileSync.mockImplementation(() => {
    throw new Error('Failed to read JSON file');
  });
  runSync(env, (value) => {
    expect(value).toEqual(O.none());
  });
});

it('returns none when file is not valid JSON', () => {
  const env = createMockEnv();
  env.readFileSync.mockReturnValue('wut?');
  runSync(env, (value) => {
    expect(value).toEqual(O.none());
  });
});

it('returns none when data is valid JSON but the wrong shape', () => {
  const env = createMockEnv();
  env.readFileSync.mockReturnValue(JSON.stringify({ packages: [1, 2] }));
  runSync(env, (value) => {
    expect(value).toEqual(O.none());
  });
});
