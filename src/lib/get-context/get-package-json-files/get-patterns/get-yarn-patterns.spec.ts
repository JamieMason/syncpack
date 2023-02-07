import { R } from '@mobily/ts-belt';
import { mockDisk } from '../../../../../test/mock-disk';
import { BaseError } from '../../../error';
import { getYarnPatterns } from './get-yarn-patterns';

describe('when Yarn config is at .workspaces[]', () => {
  it('returns an R.Ok of strings when found', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(
      JSON.stringify({ workspaces: ['a', 'b'] }),
    );
    expect(getYarnPatterns(disk)()).toEqual(R.Ok(['a', 'b']));
  });

  it('returns an R.Error when data is valid JSON but the wrong shape', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(JSON.stringify({ workspaces: [1, 2] }));
    expect(getYarnPatterns(disk)()).toEqual(
      R.Error(new BaseError('no yarn patterns found')),
    );
  });
});

describe('when Yarn config is at .workspaces.packages[]', () => {
  it('returns an R.Ok of strings when found', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(
      JSON.stringify({ workspaces: { packages: ['a', 'b'] } }),
    );
    expect(getYarnPatterns(disk)()).toEqual(R.Ok(['a', 'b']));
  });

  it('returns an R.Error when data is valid JSON but the wrong shape', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(
      JSON.stringify({ workspaces: { packages: [1, 2] } }),
    );
    expect(getYarnPatterns(disk)()).toEqual(
      R.Error(new BaseError('no yarn patterns found')),
    );
  });
});

it('returns an R.Error when disk throws', () => {
  const disk = mockDisk();
  const thrownError = new BaseError(
    'Failed to read JSON file at /fake/dir/package.json',
  );
  disk.readFileSync.mockImplementation(() => {
    throw thrownError;
  });
  expect(getYarnPatterns(disk)()).toEqual(R.Error(thrownError));
});

it('returns an R.Error when data is not valid JSON', () => {
  const disk = mockDisk();
  const thrownError = new BaseError(
    'Failed to parse JSON file at /fake/dir/package.json',
  );
  disk.readFileSync.mockReturnValue('wut?');
  expect(getYarnPatterns(disk)()).toEqual(R.Error(thrownError));
});
