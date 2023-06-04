import { Ok } from 'tightrope/result';
import { getPatterns } from '.';
import { mockEffects } from '../../../test/mock-effects';
import { DEFAULT_SOURCES } from '../../constants';
import { getContext } from '../../get-context';

it('returns new Ok of default patterns when nothing is available', () => {
  const effects = mockEffects();
  const { config } = getContext({}, effects);
  expect(getPatterns(effects)(config)).toEqual(new Ok(DEFAULT_SOURCES));
});

it('CLI --source options take precedence', () => {
  const effects = mockEffects();
  const { config } = getContext({ source: ['foo/package.json'] }, effects);
  expect(getPatterns(effects)(config)).toEqual(new Ok(['package.json', 'foo/package.json']));
});

describe('Yarn takes precedence after CLI --source options', () => {
  it('returns new Ok of strings when valid', () => {
    const effects = mockEffects();
    const { config } = getContext({}, effects);
    effects.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return JSON.stringify({ workspaces: ['yarn/*'] });
      }
    });
    expect(getPatterns(effects)(config)).toEqual(new Ok(['package.json', 'yarn/*/package.json']));
  });

  it('returns new Ok of default patterns when Yarn config is invalid', () => {
    const effects = mockEffects();
    const { config } = getContext({}, effects);
    effects.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return 'wut?';
      }
    });
    expect(getPatterns(effects)(config)).toEqual(new Ok(DEFAULT_SOURCES));
  });
});

describe('Pnpm takes precedence after Yarn', () => {
  it('returns new Ok of strings when valid', () => {
    const effects = mockEffects();
    const { config } = getContext({}, effects);
    effects.readYamlFileSync.mockImplementation(() => ({
      packages: ['pnpm/*'],
    }));
    expect(getPatterns(effects)(config)).toEqual(new Ok(['package.json', 'pnpm/*/package.json']));
  });

  it('returns new Ok of default patterns when Pnpm config is invalid', () => {
    const effects = mockEffects();
    const { config } = getContext({}, effects);
    effects.readYamlFileSync.mockImplementation(() => {
      throw new Error('Reason does not matter to this test');
    });
    expect(getPatterns(effects)(config)).toEqual(new Ok(DEFAULT_SOURCES));
  });
});

describe('Lerna takes precedence after Pnpm', () => {
  it('returns new Ok of strings when valid', () => {
    const effects = mockEffects();
    const { config } = getContext({}, effects);
    effects.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('lerna.json')) {
        return JSON.stringify({ packages: ['lerna/*'] });
      }
    });
    expect(getPatterns(effects)(config)).toEqual(new Ok(['package.json', 'lerna/*/package.json']));
  });

  it('returns new Ok of default patterns when Yarn config is invalid', () => {
    const effects = mockEffects();
    const { config } = getContext({}, effects);
    effects.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return 'wut?';
      }
    });
    expect(getPatterns(effects)(config)).toEqual(new Ok(DEFAULT_SOURCES));
  });
});
