import * as _ from 'lodash';
import { getNewest, getVersionRange, sortBySemver } from './version';

describe('getNewest', () => {
  it('returns the newest version from an array of versions', () => {
    const a = ['<1.0.0'];
    const b = _.shuffle([...a, '<=1.0.0']);
    const c = _.shuffle([...b, '1.0.0']);
    const d = _.shuffle([...c, '~1.0.0']);
    const e = _.shuffle([...d, '1.x.x']);
    const f = _.shuffle([...e, '^1.0.0']);
    const g = _.shuffle([...f, '>=1.0.0']);
    const h = _.shuffle([...g, '>1.0.0']);
    const i = _.shuffle([...h, '*']);
    const j = _.shuffle([...i, 'http://asdf.com/asdf.tar.gz']);
    const k = _.shuffle([...j, 'file:../foo/bar']);
    const l = _.shuffle([...k, 'latest']);
    const m = _.shuffle([...l, 'git+ssh://git@github.com:npm/cli.git#v1.0.27']);
    const n = _.shuffle([...m, 'git+ssh://git@github.com:npm/cli#semver:^5.0']);
    const o = _.shuffle([...n, 'git+https://isaacs@github.com/npm/cli.git']);
    const p = _.shuffle([...o, 'git://github.com/npm/cli.git#v1.0.27']);
    const q = _.shuffle([...p, 'expressjs/express']);
    const r = _.shuffle([...q, 'mochajs/mocha#4727d357ea']);
    const s = _.shuffle([...r, 'user/repo#feature/branch']);
    // valid semver
    expect(getNewest(a)).toEqual('<1.0.0');
    expect(getNewest(b)).toEqual('<=1.0.0');
    expect(getNewest(c)).toEqual('1.0.0');
    expect(getNewest(d)).toEqual('~1.0.0');
    expect(getNewest(e)).toEqual('1.x.x');
    expect(getNewest(f)).toEqual('^1.0.0');
    expect(getNewest(g)).toEqual('>=1.0.0');
    expect(getNewest(h)).toEqual('>1.0.0');
    expect(getNewest(i)).toEqual('*');
    // invalid semver
    expect(getNewest(j)).toEqual('*');
    expect(getNewest(k)).toEqual('*');
    expect(getNewest(l)).toEqual('*');
    expect(getNewest(m)).toEqual('*');
    expect(getNewest(n)).toEqual('*');
    expect(getNewest(o)).toEqual('*');
    expect(getNewest(p)).toEqual('*');
    expect(getNewest(q)).toEqual('*');
    expect(getNewest(r)).toEqual('*');
    expect(getNewest(s)).toEqual('*');
  });
});

describe('getVersionRange', () => {
  it('gets the version range from a semver dependency', () => {
    [
      ['*', '*'],
      ['>1.0.0', '>'],
      ['>=1.0.0', '>='],
      ['^1.0.0', '^'],
      ['~1.0.0', '~'],
      ['1.x.x', '.x'],
      ['1.0.0', ''],
      ['<=1.0.0', '<='],
      ['<1.0.0', '<'],
      ['http://asdf.com/asdf.tar.gz', ''],
      ['file:../foo/bar', ''],
      ['latest', ''],
      ['git+ssh://git@github.com:npm/cli.git#v1.0.27', ''],
      ['git+ssh://git@github.com:npm/cli#semver:^5.0', ''],
      ['git+https://isaacs@github.com/npm/cli.git', ''],
      ['git://github.com/npm/cli.git#v1.0.27', ''],
      ['expressjs/express"', ''],
      ['mochajs/mocha#4727d357ea"', ''],
      ['user/repo#feature/branch', '']
    ].forEach(([version, expected]) => {
      expect(getVersionRange(version)).toEqual(expected);
    });
  });
});

describe('sortBySemver', () => {
  it('returns an ordered array of semver versions', () => {
    _.times(10, () => {
      const unordered = [
        'http://asdf.com/asdf.tar.gz',
        'file:../foo/bar',
        'latest',
        'git+ssh://git@github.com:npm/cli.git#v1.0.27',
        'git+ssh://git@github.com:npm/cli#semver:^5.0',
        'git+https://isaacs@github.com/npm/cli.git',
        'git://github.com/npm/cli.git#v1.0.27',
        'expressjs/express',
        'mochajs/mocha#4727d357ea',
        'user/repo#feature/branch',
        '^1.0.0',
        '^1.0.1',
        '^1.0.0',
        '1.0.0',
        '1.0.1',
        '1.0.0',
        '*',
        '1.x.x',
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
        '1.x.x',
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
