import 'expect-more-jest';
import * as mock from '../../test/mock';
import { setSemverRange, setSemverRanges } from './set-semver-ranges';

describe('setSemverRanges', () => {
  it('sets all versions to use the supplied range', () => {
    const range = '~';
    const wrapper = mock.wrapper('a', ['foo@0.1.0', 'bar@2.0.0']);
    setSemverRanges(range, ['dependencies'], /./, wrapper);
    expect(wrapper).toEqual(mock.wrapper('a', ['foo@~0.1.0', 'bar@~2.0.0']));
  });
});

describe('setSemverRange', () => {
  describe('when the current value is Semver', () => {
    it('sets its semver range to the given range', () => {
      [
        ['', '1.2.3'],
        ['>', '>1.2.3'],
        ['>=', '>=1.2.3'],
        ['.x', '1.x.x'],
        ['<', '<1.2.3'],
        ['<=', '<=1.2.3'],
        ['^', '^1.2.3'],
        ['~', '~1.2.3'],
      ].forEach(([range, expected]) => {
        expect(setSemverRange(range, '<1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '<=1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '~1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '^1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '>=1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '>1.2.3')).toEqual(expected);
        expect(setSemverRange(range, '*')).toEqual('*');
        expect(setSemverRange(range, 'https://github.com/npm/npm.git')).toEqual('https://github.com/npm/npm.git');
      });
    });
  });
  describe('when the current value contains a wildcard patch', () => {
    it('sets its semver range to the given range', () => {
      const current = '1.2.x';
      expect(setSemverRange('', current)).toEqual('1.2.0');
      expect(setSemverRange('>', current)).toEqual('>1.2.0');
      expect(setSemverRange('>=', current)).toEqual('>=1.2.0');
      expect(setSemverRange('.x', current)).toEqual('1.x.x');
      expect(setSemverRange('<', current)).toEqual('<1.2.0');
      expect(setSemverRange('<=', current)).toEqual('<=1.2.0');
      expect(setSemverRange('^', current)).toEqual('^1.2.0');
      expect(setSemverRange('~', current)).toEqual('~1.2.0');
    });
  });
  describe('when the current value contains a wildcard minor and patch', () => {
    it('sets its semver range to the given range', () => {
      const current = '1.x.x';
      expect(setSemverRange('', current)).toEqual('1.0.0');
      expect(setSemverRange('>', current)).toEqual('>1.0.0');
      expect(setSemverRange('>=', current)).toEqual('>=1.0.0');
      expect(setSemverRange('.x', current)).toEqual(current);
      expect(setSemverRange('<', current)).toEqual('<1.0.0');
      expect(setSemverRange('<=', current)).toEqual('<=1.0.0');
      expect(setSemverRange('^', current)).toEqual('^1.0.0');
      expect(setSemverRange('~', current)).toEqual('~1.0.0');
    });
  });
  describe('when the current value contains multiple versions', () => {
    it('leaves the version unchanged', () => {
      const current = '>=16.8.0 <17.0.0';
      expect(setSemverRange('', current)).toEqual(current);
      expect(setSemverRange('>', current)).toEqual(current);
      expect(setSemverRange('>=', current)).toEqual(current);
      expect(setSemverRange('.x', current)).toEqual(current);
      expect(setSemverRange('<', current)).toEqual(current);
      expect(setSemverRange('<=', current)).toEqual(current);
      expect(setSemverRange('^', current)).toEqual(current);
      expect(setSemverRange('~', current)).toEqual(current);
    });
  });
});
