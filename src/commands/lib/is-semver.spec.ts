import 'expect-more-jest';
import { isSemver } from './is-semver';

describe('isSemver', () => {
  it('returns whether a value is Semver', () => {
    expect(isSemver('<1.2.3')).toBeTrue();
    expect(isSemver('<=1.2.3')).toBeTrue();
    expect(isSemver('1.2.3')).toBeTrue();
    expect(isSemver('1.x.x')).toBeTrue();
    expect(isSemver('1.2.x')).toBeTrue();
    expect(isSemver('~1.2.3')).toBeTrue();
    expect(isSemver('^1.2.3')).toBeTrue();
    expect(isSemver('>=1.2.3')).toBeTrue();
    expect(isSemver('>1.2.3')).toBeTrue();
    expect(isSemver('*')).toBeFalse();
    expect(isSemver('>=16.8.0 <17.0.0')).toBeFalse();
    expect(isSemver('https://github.com/npm/npm.git')).toBeFalse();
  });
});
