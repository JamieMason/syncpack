import { createPackage } from '../test/helpers';
import {
  getMismatchedVersions,
  getVersions,
  setVersion,
  setVersionRange,
  setVersionsToNewestMismatch
} from './package';
import { IPackageJson } from './typings';

let mockPackages: IPackageJson[] = [];

beforeEach(() => {
  mockPackages = [
    createPackage('foo', { jest: '22.1.3', prettier: '1.10.2', rimraf: '2.6.2' }),
    createPackage('bar', { jest: '22.1.4' }),
    createPackage('baz', { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2', glob: '*' })
  ];
});

describe('getMismatchedVersions', () => {
  it('returns an index of dependencies used with different versions', () => {
    expect(getMismatchedVersions('dependencies', mockPackages)).toEqual({
      jest: ['22.1.3', '22.1.4']
    });
  });
});

describe('getVersions', () => {
  it('returns an index of every unique dependency in use', () => {
    expect(getVersions('dependencies', mockPackages)).toEqual({
      glob: ['*'],
      jest: ['22.1.3', '22.1.4'],
      npm: ['https://github.com/npm/npm.git'],
      prettier: ['1.10.2'],
      rimraf: ['2.6.2']
    });
  });
});

describe('setVersion', () => {
  it('sets the version of a named dependency when it is found', () => {
    expect(setVersion('dependencies', 'jest', '25.0.0', mockPackages)).toEqual([
      createPackage('foo', { jest: '25.0.0', prettier: '1.10.2', rimraf: '2.6.2' }),
      createPackage('bar', { jest: '25.0.0' }),
      createPackage('baz', { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2', glob: '*' })
    ]);
  });
});

describe('setVersionRange', () => {
  it('sets the version range of all semver dependencies', () => {
    expect(setVersionRange('dependencies', '^', mockPackages)).toEqual([
      createPackage('foo', { jest: '^22.1.3', prettier: '^1.10.2', rimraf: '^2.6.2' }),
      createPackage('bar', { jest: '^22.1.4' }),
      createPackage('baz', { npm: 'https://github.com/npm/npm.git', prettier: '^1.10.2', glob: '*' })
    ]);
  });
});

describe('setVersionsToNewestMismatch', () => {
  it('sets the version of dependencies with different versions to the newest of those versions found', () => {
    expect(setVersionsToNewestMismatch('dependencies', mockPackages)).toEqual([
      createPackage('foo', { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' }),
      createPackage('bar', { jest: '22.1.4' }),
      createPackage('baz', { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2', glob: '*' })
    ]);
  });
});
