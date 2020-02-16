import { getDependencyTypes } from './get-dependency-types';

describe('getDependencyTypes', () => {
  it('returns all if none are set or true if any are set', () => {
    const prod = 'dependencies';
    const dev = 'devDependencies';
    const peer = 'peerDependencies';
    expect(getDependencyTypes({ dev: false, peer: false, prod: false })).toEqual([prod, dev, peer]);
    expect(getDependencyTypes({ dev: true, peer: false, prod: false })).toEqual([dev]);
    expect(getDependencyTypes({ dev: false, peer: true, prod: false })).toEqual([peer]);
    expect(getDependencyTypes({ dev: false, peer: false, prod: true })).toEqual([prod]);
  });
});
