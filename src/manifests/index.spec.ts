import { readFileSync } from 'fs';
import * as mock from 'mock-fs';
import { createFile, createManifest, createMockDescriptor, createMockFs } from '../../test/helpers';
import { IManifest, IManifestDescriptor } from '../typings';
import { getMismatchedVersions, getVersions, setVersion, setVersionRange, setVersionsToNewestMismatch } from './index';

const pattern = '/Users/you/Dev/monorepo/packages/*/package.json';

afterEach(() => {
  mock.restore();
});

beforeEach(() => {
  mock({
    ...createMockFs(
      'foo',
      { chalk: '2.3.0', commander: '2.13.0' },
      { jest: '22.1.3', prettier: '1.10.2', rimraf: '2.6.2' },
      { gulp: '3.9.1' }
    ),
    ...createMockFs('bar', { chalk: '1.0.0' }, { jest: '22.1.4' }),
    ...createMockFs('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
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
        createMockDescriptor(
          'foo',
          { chalk: '2.3.0', commander: '2.13.0' },
          { jest: '25.0.0', prettier: '1.10.2', rimraf: '2.6.2' },
          { gulp: '3.9.1' }
        ),
        createMockDescriptor('bar', { chalk: '1.0.0' }, { jest: '25.0.0' }),
        createMockDescriptor('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
      ])
    );
  });
});

describe('setVersionRange', () => {
  it('sets the version range of all semver dependencies', async () => {
    const result = await setVersionRange('^', pattern);
    expect(result).toEqual(
      expect.arrayContaining([
        createMockDescriptor(
          'foo',
          { chalk: '^2.3.0', commander: '^2.13.0' },
          { jest: '^22.1.3', prettier: '^1.10.2', rimraf: '^2.6.2' },
          { gulp: '^3.9.1' }
        ),
        createMockDescriptor('bar', { chalk: '^1.0.0' }, { jest: '^22.1.4' }),
        createMockDescriptor('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '^1.10.2' }, { gulp: '*' })
      ])
    );
  });
});

describe('setVersionsToNewestMismatch', () => {
  it('sets all dependencies used with different versions to the newest of those versions', async () => {
    const result = await setVersionsToNewestMismatch(pattern);
    expect(result).toEqual(
      expect.arrayContaining([
        createMockDescriptor(
          'foo',
          { chalk: '2.3.0', commander: '2.13.0' },
          { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' },
          { gulp: '*' }
        ),
        createMockDescriptor('bar', { chalk: '2.3.0' }, { jest: '22.1.4' }),
        createMockDescriptor('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
      ])
    );
  });
  it('rewrites the updated manifests with the correct data', async () => {
    await setVersionsToNewestMismatch(pattern);
    expect(readFileSync('/Users/you/Dev/monorepo/packages/foo/package.json', 'utf8')).toEqual(
      createFile(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '*' }
      )
    );
    expect(readFileSync('/Users/you/Dev/monorepo/packages/bar/package.json', 'utf8')).toEqual(
      createFile('bar', { chalk: '2.3.0' }, { jest: '22.1.4' })
    );
    expect(readFileSync('/Users/you/Dev/monorepo/packages/baz/package.json', 'utf8')).toEqual(
      createFile('baz', {}, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    );
  });
});
