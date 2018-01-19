import { createPackage } from '../test/helpers';
import {
  getMismatchedVersions,
  getVersions,
  setVersion,
  setVersionRange,
  setVersionsToNewestMismatch
} from './packages';
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
    expect(getMismatchedVersions(mockPackages)).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      gulp: ['*', '3.9.1'],
      jest: ['22.1.3', '22.1.4']
    });
  });
});

describe.skip('getVersions', () => {
  it('returns an index of every unique dependency in use', () => {
    expect(getVersions(mockPackages)).toEqual({
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

describe.skip('setVersion', () => {
  it('sets the version of a named dependency when it is found', () => {
    expect(setVersion('jest', '25.0.0', mockPackages)).toEqual([
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

describe.skip('setVersionRange', () => {
  it('sets the version range of all semver dependencies', () => {
    expect(setVersionRange('^', mockPackages)).toEqual([
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

describe.skip('setVersionsToNewestMismatch', () => {
  it('sets the version of dependencies with different versions to the newest of those versions found', () => {
    expect(setVersionsToNewestMismatch(mockPackages)).toEqual([
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
