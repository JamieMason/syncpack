import { Err, Ok } from 'tightrope/result';
import { mockEffects } from '../../../test/mock-effects';
import { getPnpmPatterns } from './get-pnpm-patterns';

it('returns an new Ok of strings when found', () => {
  const effects = mockEffects();
  effects.readYamlFileSync.mockReturnValue({ packages: ['a', 'b'] });
  expect(getPnpmPatterns(effects)()).toEqual(new Ok(['a', 'b']));
});

it('returns an new Err when effects throws', () => {
  const effects = mockEffects();
  effects.readYamlFileSync.mockImplementation(() => {
    throw new Error('Failed to read YAML file');
  });
  expect(getPnpmPatterns(effects)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is valid YAML but the wrong shape', () => {
  const effects = mockEffects();
  effects.readYamlFileSync.mockReturnValue({ packages: [1, 2] });
  expect(getPnpmPatterns(effects)()).toEqual(expect.any(Err));
});
