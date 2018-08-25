import _ = require('lodash');
import { getNewest, getVersionRange, sortBySemver } from './version';

describe('getNewest', () => {
  it('returns the newest version from an array of versions', () => {
    const a = ['<1.0.0'];
    const b = _.shuffle(a.concat('<=1.0.0'));
    const c = _.shuffle(b.concat('1.0.0'));
    const d = _.shuffle(c.concat('~1.0.0'));
    const e = _.shuffle(d.concat('1.x.x'));
    const f = _.shuffle(e.concat('^1.0.0'));
    const g = _.shuffle(f.concat('>=1.0.0'));
    const h = _.shuffle(g.concat('>1.0.0'));
    const i = _.shuffle(h.concat('*'));
    expect(getNewest(a)).toEqual('<1.0.0');
    expect(getNewest(b)).toEqual('<=1.0.0');
    expect(getNewest(c)).toEqual('1.0.0');
    expect(getNewest(d)).toEqual('~1.0.0');
    expect(getNewest(e)).toEqual('1.x.x');
    expect(getNewest(f)).toEqual('^1.0.0');
    expect(getNewest(g)).toEqual('>=1.0.0');
    expect(getNewest(h)).toEqual('>1.0.0');
    expect(getNewest(i)).toEqual('*');
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
      const unordered = [
        '^1.0.0',
        '^1.0.1',
        '^1.0.0',
        '1.0.0',
        '1.0.1',
        '1.0.0',
        '*',
        '*',
        '<=1.0.0',
        '<=1.0.1',
        '<=1.0.0',
        '>1.0.0',
        '>1.0.1',
        '>1.0.0',
        '>=1.0.0',
        '>=1.0.1',
        '>=1.0.0',
        '<1.0.0',
        '<1.0.1',
        '<1.0.0',
        '~1.0.0',
        '~1.0.1',
        '~1.0.0'
      ];
      const expectedOrder = [
        '<1.0.0',
        '<1.0.0',
        '<=1.0.0',
        '<=1.0.0',
        '1.0.0',
        '1.0.0',
        '~1.0.0',
        '~1.0.0',
        '^1.0.0',
        '^1.0.0',
        '>=1.0.0',
        '>=1.0.0',
        '>1.0.0',
        '>1.0.0',
        '<1.0.1',
        '<=1.0.1',
        '1.0.1',
        '~1.0.1',
        '^1.0.1',
        '>=1.0.1',
        '>1.0.1',
        '*',
        '*'
      ];
      const shuffled = _.shuffle(unordered);
      expect(sortBySemver(shuffled)).toEqual(expectedOrder);
    });
  });
});
