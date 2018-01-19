import * as _ from 'lodash';
import { getNewest, getVersionRange, sortBySemver } from './version';

describe('getNewest', () => {
  it('returns the newest version from an array of versions', () => {
    expect(getNewest(['<=1.0.0', '<1.0.0'])).toEqual('<=1.0.0');
    expect(getNewest(['1.0.0', '<=1.0.0', '<1.0.0'])).toEqual('1.0.0');
    expect(getNewest(['~1.0.0', '1.0.0', '<=1.0.0', '<1.0.0'])).toEqual('~1.0.0');
    expect(getNewest(['^1.0.0', '~1.0.0', '1.0.0', '<=1.0.0', '<1.0.0'])).toEqual('^1.0.0');
    expect(getNewest(['>=1.0.0', '^1.0.0', '~1.0.0', '1.0.0', '<=1.0.0', '<1.0.0'])).toEqual('>=1.0.0');
    expect(getNewest(['>1.0.0', '>=1.0.0', '^1.0.0', '~1.0.0', '1.0.0', '<=1.0.0', '<1.0.0'])).toEqual('>1.0.0');
    expect(getNewest(['>1.0.0', '>=1.0.0', '^1.0.0', '*', '~1.0.0', '1.0.0', '<=1.0.0', '<1.0.0'])).toEqual('*');
  });
});

describe('getVersionRange', () => {
  it('gets the version range from a semver dependency', () => {
    expect(getVersionRange('*')).toEqual('*');
    expect(getVersionRange('>1.0.0')).toEqual('>');
    expect(getVersionRange('>=1.0.0')).toEqual('>=');
    expect(getVersionRange('^1.0.0')).toEqual('^');
    expect(getVersionRange('~1.0.0')).toEqual('~');
    expect(getVersionRange('1.0.0')).toEqual('');
    expect(getVersionRange('<=1.0.0')).toEqual('<=');
    expect(getVersionRange('<1.0.0')).toEqual('<');
  });
});

describe('sortBySemver', () => {
  it('returns an ordered array of semver versions', () => {
    _.times(10, () => {
      const unordered = ['^1.0.0', '1.0.0', '*', '<=1.0.0', '>1.0.0', '>=1.0.0', '<1.0.0', '~1.0.0'];
      const expectedOrder = ['<1.0.0', '<=1.0.0', '1.0.0', '~1.0.0', '^1.0.0', '>=1.0.0', '>1.0.0', '*'];
      const shuffled = _.shuffle(unordered);
      expect(sortBySemver(shuffled)).toEqual(expectedOrder);
    });
  });
});
