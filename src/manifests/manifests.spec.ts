import * as mock from 'mock-fs';
import { createManifest, createMockProject } from '../../test/helpers';
import { IManifest } from '../typings';
import { getMismatchedVersions, getVersions, setVersion, setVersionRange, setVersionsToNewestMismatch } from './index';

const pattern = '/Users/you/Dev/monorepo/packages/*/package.json';

afterEach(() => {
  mock.restore();
});

beforeEach(() => {
  mock({
    ...createMockProject(
      'foo',
      { chalk: '2.3.0', commander: '2.13.0' },
      { jest: '22.1.3', prettier: '1.10.2', rimraf: '2.6.2' },
      { gulp: '3.9.1' }
    ),
    ...createMockProject('bar', { chalk: '1.0.0' }, { jest: '22.1.4' }),
    ...createMockProject('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
  });
});

describe('getMismatchedVersions', () => {
  it('returns an index of dependencies used with different versions', async () => {
    const result = await getMismatchedVersions(pattern);
    expect(result).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      gulp: ['*', '3.9.1'],
      jest: ['22.1.3', '22.1.4']
    });
  });
});

describe('getVersions', () => {
  it('returns an index of every unique dependency in use', async () => {
    const result = await getVersions(pattern);
    expect(result).toEqual({
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
  it('sets the version of a named dependency when it is found', async () => {
    const result = await setVersion('jest', '25.0.0', pattern);
    expect(result).toEqual(
      expect.arrayContaining([
        createManifest(
          'foo',
          { chalk: '2.3.0', commander: '2.13.0' },
          { jest: '25.0.0', prettier: '1.10.2', rimraf: '2.6.2' },
          { gulp: '3.9.1' }
        ),
        createManifest('bar', { chalk: '1.0.0' }, { jest: '25.0.0' }),
        createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
      ])
    );
  });
});

describe('setVersionRange', () => {
  it('sets the version range of all semver dependencies', async () => {
    const result = await setVersionRange('^', pattern);
    expect(result).toEqual(
      expect.arrayContaining([
        createManifest(
          'foo',
          { chalk: '^2.3.0', commander: '^2.13.0' },
          { jest: '^22.1.3', prettier: '^1.10.2', rimraf: '^2.6.2' },
          { gulp: '^3.9.1' }
        ),
        createManifest('bar', { chalk: '^1.0.0' }, { jest: '^22.1.4' }),
        createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '^1.10.2' }, { gulp: '*' })
      ])
    );
  });
});

describe('setVersionsToNewestMismatch', () => {
  it('sets the version of dependencies with different versions to the newest of those versions found', async () => {
    const result = await setVersionsToNewestMismatch(pattern);
    expect(result).toEqual(
      expect.arrayContaining([
        createManifest(
          'foo',
          { chalk: '2.3.0', commander: '2.13.0' },
          { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' },
          { gulp: '*' }
        ),
        createManifest('bar', { chalk: '2.3.0' }, { jest: '22.1.4' }),
        createManifest('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
      ])
    );
  });
});
