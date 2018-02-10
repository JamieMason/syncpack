import {
  getAnyVersionManifests,
  getExactVersionManifests,
  getGreaterThanOrEqualVersionManifests,
  getGreaterThanVersionManifests,
  getLessThanOrEqualVersionManifests,
  getLessThanVersionManifests,
  getLooseVersionManifests,
  getMinorVersionManifests,
  getPatchVersionManifests
} from '../../../test/fixtures';
import { createManifest } from '../../../test/helpers';
import {
  RANGE_ANY,
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LOOSE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH
} from '../../constants';
import { IManifest } from '../../typings';
import { manifestData } from './index';

const { getMismatchedVersions, getVersions, setVersion, setVersionRange, setVersionsToNewestMismatch } = manifestData;

describe('getMismatchedVersions', () => {
  it('returns an index of dependencies used with different versions', () => {
    expect(getMismatchedVersions(getExactVersionManifests())).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      gulp: ['*', '0.9.1'],
      jest: ['22.1.3', '22.1.4']
    });
  });
});

describe('getVersions', () => {
  it('returns an index of every unique dependency in use', () => {
    expect(getVersions(getExactVersionManifests())).toEqual({
      chalk: ['1.0.0', '2.3.0'],
      commander: ['2.13.0'],
      gulp: ['*', '0.9.1'],
      jest: ['22.1.3', '22.1.4'],
      npm: ['https://github.com/npm/npm.git'],
      prettier: ['1.10.2'],
      rimraf: ['2.6.2']
    });
  });
});

describe('setVersion', () => {
  it('sets the version of a named dependency when it is found', () => {
    expect(setVersion('jest', '25.0.0', getExactVersionManifests())).toEqual([
      createManifest(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '25.0.0', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '0.9.1' }
      ),
      createManifest('bar', { chalk: '1.0.0' }, { jest: '25.0.0' }),
      createManifest('baz', null, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    ]);
  });
});

describe('setVersionRange', () => {
  const assertRange = (inputRange: string, expectedManifests: IManifest[]) => {
    [
      getExactVersionManifests(),
      getGreaterThanVersionManifests(),
      getGreaterThanOrEqualVersionManifests(),
      getLessThanVersionManifests(),
      getLessThanOrEqualVersionManifests(),
      getMinorVersionManifests(),
      getPatchVersionManifests()
    ].forEach((inputManifests) => {
      expect(setVersionRange(inputRange, inputManifests)).toEqual(expectedManifests);
    });
  };

  it(`sets semver ranges to the "${RANGE_ANY}" format`, () => {
    assertRange(RANGE_ANY, getAnyVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_EXACT}" format`, () => {
    assertRange(RANGE_EXACT, getExactVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_GT}" format`, () => {
    assertRange(RANGE_GT, getGreaterThanVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_GTE}" format`, () => {
    assertRange(RANGE_GTE, getGreaterThanOrEqualVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_LOOSE}" format`, () => {
    assertRange(RANGE_LOOSE, getLooseVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_LT}" format`, () => {
    assertRange(RANGE_LT, getLessThanVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_LTE}" format`, () => {
    assertRange(RANGE_LTE, getLessThanOrEqualVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_MINOR}" format`, () => {
    assertRange(RANGE_MINOR, getMinorVersionManifests());
  });
  it(`sets semver ranges to the "${RANGE_PATCH}" format`, () => {
    assertRange(RANGE_PATCH, getPatchVersionManifests());
  });
});

describe('setVersionsToNewestMismatch', () => {
  it('sets the version of dependencies with different versions to the newest of those versions found', () => {
    expect(setVersionsToNewestMismatch(getExactVersionManifests())).toEqual([
      createManifest(
        'foo',
        { chalk: '2.3.0', commander: '2.13.0' },
        { jest: '22.1.4', prettier: '1.10.2', rimraf: '2.6.2' },
        { gulp: '*' }
      ),
      createManifest('bar', { chalk: '2.3.0' }, { jest: '22.1.4' }),
      createManifest('baz', null, { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' }, { gulp: '*' })
    ]);
  });
});
