import { describe, expect, it } from 'vitest';
import type { SemverRange } from '../config/types.js';
import { setSemverRange } from './set-semver-range.js';

describe('setSemverRange', () => {
  describe('when the current value is Semver', () => {
    it('sets its semver range to the given range', async () => {
      const cases: [SemverRange, string][] = [
        ['*', '*'],
        ['', '1.2.3'],
        ['>', '>1.2.3'],
        ['>=', '>=1.2.3'],
        ['.x', '1.x.x'],
        ['<', '<1.2.3'],
        ['<=', '<=1.2.3'],
        ['^', '^1.2.3'],
        ['~', '~1.2.3'],
      ];
      cases.forEach(([semverRange, expected]) => {
        expect(setSemverRange(semverRange, '<1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '<=1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '~1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '^1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '>=1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '>1.2.3')).toEqual(expected);
        expect(setSemverRange(semverRange, '*')).toEqual('*');
        expect(setSemverRange(semverRange, 'https://github.com/npm/npm.git')).toEqual('https://github.com/npm/npm.git');
      });
    });
  });

  describe('when the current value contains a wildcard patch', () => {
    it('sets its semver range to the given range', async () => {
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
    it('sets its semver range to the given range', async () => {
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
    it('leaves the version unchanged', async () => {
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
