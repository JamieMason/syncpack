import { Err, Ok } from 'tightrope/result';
import { mockEffects } from '../../../test/mock-effects';
import { getLernaPatterns } from './get-lerna-patterns';

it('returns an new Ok of strings when found', () => {
  const effects = mockEffects();
  effects.readFileSync.mockReturnValue(JSON.stringify({ packages: ['a', 'b'] }));
  expect(getLernaPatterns(effects)()).toEqual(new Ok(['a', 'b']));
});

it('returns an new Err when effects throws', () => {
  const effects = mockEffects();
  effects.readFileSync.mockImplementation(() => {
    throw new Error('Failed to read JSON file');
  });
  expect(getLernaPatterns(effects)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is not valid JSON', () => {
  const effects = mockEffects();
  effects.readFileSync.mockReturnValue('wut?');
  expect(getLernaPatterns(effects)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is valid JSON but the wrong shape', () => {
  const effects = mockEffects();
  effects.readFileSync.mockReturnValue(JSON.stringify({ packages: [1, 2] }));
  expect(getLernaPatterns(effects)()).toEqual(expect.any(Err));
});
