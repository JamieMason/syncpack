import { match } from '@effect/data/Either';
import { identity, pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import type { MockEnv } from '../../../test/mock-env';
import { createMockEnv } from '../../../test/mock-env';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import { getPnpmPatterns } from './get-pnpm-patterns';

function runSync(mockedEffects: MockEnv) {
  return pipe(
    Effect.runSyncEither(
      pipe(getPnpmPatterns(), Effect.provideService(EnvTag, createEnv(mockedEffects))),
    ),
    match(identity, identity),
  );
}

it('returns strings when found', () => {
  const env = createMockEnv();
  env.readYamlFileSync.mockReturnValue({ packages: ['a', 'b'] });
  expect(runSync(env)).toEqual(O.some(['a', 'b']));
});

it('returns none when a file cannot be read', () => {
  const env = createMockEnv();
  env.readYamlFileSync.mockImplementation(() => {
    throw new Error('Failed to read YAML file');
  });
  expect(runSync(env)).toEqual(O.none());
});

it('returns none when data is valid YAML but the wrong shape', () => {
  const env = createMockEnv();
  env.readYamlFileSync.mockReturnValue({ packages: [1, 2] });
  expect(runSync(env)).toEqual(O.none());
});
