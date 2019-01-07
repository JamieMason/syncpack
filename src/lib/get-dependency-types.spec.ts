import { CommanderApi } from '../typings';
import { getDependencyTypes } from './get-dependency-types';

describe('getDependencyTypes', () => {
  it('returns all if none are set or true if any are set', () => {
    [
      [
        { dev: false, peer: false, prod: false },
        ['dependencies', 'devDependencies', 'peerDependencies']
      ],
      [{ dev: true, peer: false, prod: false }, ['devDependencies']],
      [{ dev: false, peer: true, prod: false }, ['peerDependencies']],
      [{ dev: false, peer: false, prod: true }, ['dependencies']]
    ].forEach(([program, expected]) => {
      expect(getDependencyTypes((program as unknown) as CommanderApi)).toEqual(
        expected
      );
    });
  });
});
