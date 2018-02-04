import { createManifest } from '../../../test/helpers';
import { IManifest } from '../../typings';
import { manifestData } from './index';

const { getMismatchedVersions, getVersions, setVersion, setVersionRange, setVersionsToNewestMismatch } = manifestData;

let mockManifests: IManifest[] = [];

beforeEach(() => {
  mockManifests = [
    createManifest(
      'foo',
      { chalk: '2.3.0', commander: '2.13.0' },
      { jest: '22.1.3', prettier: '1.10.2', rimraf: '2.6.2' },
      { gulp: '3.9.1' }
    ),
    createManifest('bar', { chalk: '1.0.0' }, { jest: '22.1.4' }),
    createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
  ];
});

describe('getMismatchedVersions', () => {
  it('returns an index of dependencies used with different versions', () => {
    expect(getMismatchedVersions(mockManifests)).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      gulp: ['*', '3.9.1'],
      jest: ['22.1.3', '22.1.4']
    });
  });
});

describe('getVersions', () => {
  it('returns an index of every unique dependency in use', () => {
    expect(getVersions(mockManifests)).toEqual({
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
    expect(setVersion('jest', '25.0.0', mockManifests)).toEqual([
      createManifest(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '25.0.0', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '3.9.1' }
      ),
      createManifest('bar', { chalk: '1.0.0' }, { jest: '25.0.0' }),
      createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    ]);
  });
});

describe('setVersionRange', () => {
  it('sets the version range of all semver dependencies', () => {
    expect(setVersionRange('^', mockManifests)).toEqual([
      createManifest(
        'foo',
        { chalk: '^2.3.0', commander: '^2.13.0' },
        { jest: '^22.1.3', prettier: '^1.10.2', rimraf: '^2.6.2' },
        { gulp: '^3.9.1' }
      ),
      createManifest('bar', { chalk: '^1.0.0' }, { jest: '^22.1.4' }),
      createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '^1.10.2' }, { gulp: '*' })
    ]);
  });
});

describe('setVersionsToNewestMismatch', () => {
  it('sets the version of dependencies with different versions to the newest of those versions found', () => {
    expect(setVersionsToNewestMismatch(mockManifests)).toEqual([
      createManifest(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '*' }
      ),
      createManifest('bar', { chalk: '2.3.0' }, { jest: '22.1.4' }),
      createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    ]);
  });
});
