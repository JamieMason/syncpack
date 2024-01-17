import { describe, expect, it } from 'vitest';
import { isSemver } from './is-semver.js';

describe('isSemver', () => {
  it('returns whether a value is Semver', async () => {
    expect(isSemver('<1.2.3')).toEqual(true);
    expect(isSemver('<=1.2.3')).toEqual(true);
    expect(isSemver('1.2.3')).toEqual(true);
    expect(isSemver('1.x.x')).toEqual(true);
    expect(isSemver('1.2.x')).toEqual(true);
    expect(isSemver('~1.2.3')).toEqual(true);
    expect(isSemver('^1.2.3')).toEqual(true);
    expect(isSemver('>=1.2.3')).toEqual(true);
    expect(isSemver('>1.2.3')).toEqual(true);
    expect(isSemver('>1')).toEqual(true);
    expect(isSemver('>=1')).toEqual(true);
    expect(isSemver('^1')).toEqual(true);
    expect(isSemver('*')).toEqual(false);
    expect(isSemver('>=16.8.0 <17.0.0')).toEqual(false);
    expect(isSemver('https://github.com/npm/npm.git')).toEqual(false);
  });
});
