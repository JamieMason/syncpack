import { createPackage } from '../test/helpers';
import {
  getMismatchedPackageVersions,
  getPackageVersions,
  setPackageVersion,
  setPackageVersionRange,
  setPackageVersionsToNewestMismatch
} from './package';
import { IPackageJson } from './typings';

let mockPackages: IPackageJson[] = [];

beforeEach(() => {
  mockPackages = [
    createPackage(
      'foo',
      { chalk: '2.3.0', commander: '2.13.0' },
      { jest: '22.1.3', prettier: '1.10.2', rimraf: '2.6.2' },
      { gulp: '3.9.1' }
    ),
    createPackage('bar', { chalk: '1.0.0' }, { jest: '22.1.4' }),
    createPackage('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
  ];
});

describe('getMismatchedVersions', () => {
  it('returns an index of dependencies used with different versions', () => {
    expect(getMismatchedPackageVersions(mockPackages)).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      gulp: ['*', '3.9.1'],
      jest: ['22.1.3', '22.1.4']
    });
  });
});

describe('getVersions', () => {
  it('returns an index of every unique dependency in use', () => {
    expect(getPackageVersions(mockPackages)).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      commander: ['2.13.0'],
      gulp: ['*', '3.9.1'],
      jest: ['22.1.3', '22.1.4'],
      npm: ['https://github.com/npm/npm.git'],
      prettier: ['1.10.2'],
      rimraf: ['2.6.2']
    });
  });
});

describe('setVersion', () => {
  it('sets the version of a named dependency when it is found', () => {
    expect(setPackageVersion('jest', '25.0.0', mockPackages)).toEqual([
      createPackage(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '25.0.0', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '3.9.1' }
      ),
      createPackage('bar', { chalk: '1.0.0' }, { jest: '25.0.0' }),
      createPackage('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    ]);
  });
});

describe('setVersionRange', () => {
  it('sets the version range of all semver dependencies', () => {
    expect(setPackageVersionRange('^', mockPackages)).toEqual([
      createPackage(
        'foo',
        { chalk: '^2.3.0', commander: '^2.13.0' },
        { jest: '^22.1.3', prettier: '^1.10.2', rimraf: '^2.6.2' },
        { gulp: '^3.9.1' }
      ),
      createPackage('bar', { chalk: '^1.0.0' }, { jest: '^22.1.4' }),
      createPackage('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '^1.10.2' }, { gulp: '*' })
    ]);
  });
});

describe('setVersionsToNewestMismatch', () => {
  it('sets the version of dependencies with different versions to the newest of those versions found', () => {
    expect(setPackageVersionsToNewestMismatch(mockPackages)).toEqual([
      createPackage(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '*' }
      ),
      createPackage('bar', { chalk: '2.3.0' }, { jest: '22.1.4' }),
      createPackage('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    ]);
  });
});
