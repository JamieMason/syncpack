import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import type { MockEnv } from '../../../test/mock-env';
import { createMockEnv } from '../../../test/mock-env';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import { getYarnPatterns } from './get-yarn-patterns';

function runSync(mockedEffects: MockEnv, onValue: (value: any) => void) {
  Effect.runSync(
    pipe(
      getYarnPatterns(),
      Effect.match({
        onFailure: onValue,
        onSuccess: onValue,
      }),
      Effect.provideService(EnvTag, createEnv(mockedEffects)),
    ),
  );
}

describe('when Yarn config is at .workspaces[]', () => {
  it('returns strings when found', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: ['a', 'b'] }));
    runSync(env, (value) => {
      expect(value).toEqual(O.some(['a', 'b']));
    });
  });

  it('returns none when data is valid JSON but the wrong shape', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: [1, 2] }));
    runSync(env, (value) => {
      expect(value).toEqual(O.none());
    });
  });
});

describe('when Yarn config is at .workspaces.packages[]', () => {
  it('returns an strings when found', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: { packages: ['a', 'b'] } }));
    runSync(env, (value) => {
      expect(value).toEqual(O.some(['a', 'b']));
    });
  });

  it('returns none when data is valid JSON but the wrong shape', () => {
    const env = createMockEnv();
    env.readFileSync.mockReturnValue(JSON.stringify({ workspaces: { packages: [1, 2] } }));
    runSync(env, (value) => {
      expect(value).toEqual(O.none());
    });
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
