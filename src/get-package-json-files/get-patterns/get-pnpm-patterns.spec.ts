import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import type { MockEnv } from '../../../test/lib/mock-env';
import { createMockEnv } from '../../../test/lib/mock-env';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import { getPnpmPatterns } from './get-pnpm-patterns';

function runSync(mockedEffects: MockEnv, onValue: (value: any) => void) {
  Effect.runSync(
    pipe(
      getPnpmPatterns(),
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
  env.readYamlFileSync.mockReturnValue({ packages: ['a', 'b'] });
  runSync(env, (value) => {
    expect(value).toEqual(O.some(['a', 'b']));
  });
});

it('returns none when a file cannot be read', () => {
  const env = createMockEnv();
  env.readYamlFileSync.mockImplementation(() => {
    throw new Error('Failed to read YAML file');
  });
  runSync(env, (value) => {
    expect(value).toEqual(O.none());
  });
});

it('returns none when data is valid YAML but the wrong shape', () => {
  const env = createMockEnv();
  env.readYamlFileSync.mockReturnValue({ packages: [1, 2] });
  runSync(env, (value) => {
    expect(value).toEqual(O.none());
  });
});
