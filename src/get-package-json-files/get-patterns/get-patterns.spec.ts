import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import { getPatterns } from '.';
import type { MockEnv } from '../../../test/mock-env';
import { createMockEnv } from '../../../test/mock-env';
import { DEFAULT_SOURCES } from '../../constants';
import { createEnv } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import type { Ctx } from '../../get-context';

function runSync(config: Ctx['config'], mockedEffects: MockEnv) {
  return pipe(
    Effect.runSync(
      pipe(getPatterns(config), Effect.provideService(EnvTag, createEnv(mockedEffects))),
    ),
  );
}

it('returns default patterns when nothing is available', () => {
  const env = createMockEnv();
  const result = runSync({ cli: {}, rcFile: {} }, env);
  expect(result).toEqual(DEFAULT_SOURCES);
});

it('CLI --source options take precedence', () => {
  const env = createMockEnv();
  const result = runSync({ cli: { source: ['foo/package.json'] }, rcFile: {} }, env);
  expect(result).toEqual(['package.json', 'foo/package.json']);
});

describe('Yarn takes precedence after CLI --source options', () => {
  it('returns strings when valid', () => {
    const env = createMockEnv();
    env.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return JSON.stringify({ workspaces: ['yarn/*'] });
      }
    });
    const result = runSync({ cli: {}, rcFile: {} }, env);
    expect(result).toEqual(['package.json', 'yarn/*/package.json']);
  });

  it('returns default patterns when Yarn config is invalid', () => {
    const env = createMockEnv();
    env.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return 'wut?';
      }
    });
    const result = runSync({ cli: {}, rcFile: {} }, env);
    expect(result).toEqual(DEFAULT_SOURCES);
  });
});

describe('Pnpm takes precedence after Yarn', () => {
  it('returns strings when valid', () => {
    const env = createMockEnv();
    env.readYamlFileSync.mockImplementation(() => ({
      packages: ['pnpm/*'],
    }));
    const result = runSync({ cli: {}, rcFile: {} }, env);
    expect(result).toEqual(['package.json', 'pnpm/*/package.json']);
  });

  it('returns default patterns when Pnpm config is invalid', () => {
    const env = createMockEnv();
    env.readYamlFileSync.mockImplementation(() => {
      throw new Error('Reason does not matter to this test');
    });
    const result = runSync({ cli: {}, rcFile: {} }, env);
    expect(result).toEqual(DEFAULT_SOURCES);
  });
});

describe('Lerna takes precedence after Pnpm', () => {
  it('returns strings when valid', () => {
    const env = createMockEnv();
    env.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('lerna.json')) {
        return JSON.stringify({ packages: ['lerna/*'] });
      }
    });
    const result = runSync({ cli: {}, rcFile: {} }, env);
    expect(result).toEqual(['package.json', 'lerna/*/package.json']);
  });

  it('returns default patterns when Yarn config is invalid', () => {
    const env = createMockEnv();
    env.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return 'wut?';
      }
    });
    const result = runSync({ cli: {}, rcFile: {} }, env);
    expect(result).toEqual(DEFAULT_SOURCES);
  });
});
