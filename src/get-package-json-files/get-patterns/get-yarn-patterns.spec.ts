import { Err, Ok } from 'tightrope/result';
import { mockEffects } from '../../../test/mock-effects';
import { getYarnPatterns } from './get-yarn-patterns';

describe('when Yarn config is at .workspaces[]', () => {
  it('returns an new Ok of strings when found', () => {
    const effects = mockEffects();
    effects.readFileSync.mockReturnValue(JSON.stringify({ workspaces: ['a', 'b'] }));
    expect(getYarnPatterns(effects)()).toEqual(new Ok(['a', 'b']));
  });

  it('returns an new Err when data is valid JSON but the wrong shape', () => {
    const effects = mockEffects();
    effects.readFileSync.mockReturnValue(JSON.stringify({ workspaces: [1, 2] }));
    expect(getYarnPatterns(effects)()).toEqual(expect.any(Err));
  });
});

describe('when Yarn config is at .workspaces.packages[]', () => {
  it('returns an new Ok of strings when found', () => {
    const effects = mockEffects();
    effects.readFileSync.mockReturnValue(JSON.stringify({ workspaces: { packages: ['a', 'b'] } }));
    expect(getYarnPatterns(effects)()).toEqual(new Ok(['a', 'b']));
  });

  it('returns an new Err when data is valid JSON but the wrong shape', () => {
    const effects = mockEffects();
    effects.readFileSync.mockReturnValue(JSON.stringify({ workspaces: { packages: [1, 2] } }));
    expect(getYarnPatterns(effects)()).toEqual(expect.any(Err));
  });
});

it('returns an new Err when effects throws', () => {
  const effects = mockEffects();
  effects.readFileSync.mockImplementation(() => {
    throw new Error('Failed to read JSON file');
  });
  expect(getYarnPatterns(effects)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is not valid JSON', () => {
  const effects = mockEffects();
  effects.readFileSync.mockReturnValue('wut?');
  expect(getYarnPatterns(effects)()).toEqual(expect.any(Err));
});
