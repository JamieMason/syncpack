import { match } from '@effect/data/Either';
import { identity, pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import type { MockEnv } from '../../../test/mock-env';
import { createMockEnv } from '../../../test/mock-env';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import { getYarnPatterns } from './get-yarn-patterns';

function runSync(mockedEffects: MockEnv) {
  return pipe(
    Effect.runSyncEither(
      pipe(getYarnPatterns(), Effect.provideService(EnvTag, createEnv(mockedEffects))),
    ),
    match(identity, identity),
  );
}

describe('when Yarn config is at .workspaces[]', () => {
  it('returns strings when found', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: ['a', 'b'] }));
    expect(runSync(env)).toEqual(O.some(['a', 'b']));
  });

  it('returns none when data is valid JSON but the wrong shape', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: [1, 2] }));
    expect(runSync(env)).toEqual(O.none());
  });
});

describe('when Yarn config is at .workspaces.packages[]', () => {
  it('returns an strings when found', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: { packages: ['a', 'b'] } }));
    expect(runSync(env)).toEqual(O.some(['a', 'b']));
  });

  it('returns none when data is valid JSON but the wrong shape', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: { packages: [1, 2] } }));
    expect(runSync(env)).toEqual(O.none());
  });
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
