import { Err, Ok } from 'tightrope/result';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { getYarnPatterns } from './get-yarn-patterns';

describe('when Yarn config is at .workspaces[]', () => {
  it('returns an new Ok of strings when found', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(
      JSON.stringify({ workspaces: ['a', 'b'] }),
    );
    expect(getYarnPatterns(disk)()).toEqual(new Ok(['a', 'b']));
  });

  it('returns an new Err when data is valid JSON but the wrong shape', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(JSON.stringify({ workspaces: [1, 2] }));
    expect(getYarnPatterns(disk)()).toEqual(expect.any(Err));
  });
});

describe('when Yarn config is at .workspaces.packages[]', () => {
  it('returns an new Ok of strings when found', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(
      JSON.stringify({ workspaces: { packages: ['a', 'b'] } }),
    );
    expect(getYarnPatterns(disk)()).toEqual(new Ok(['a', 'b']));
  });

  it('returns an new Err when data is valid JSON but the wrong shape', () => {
    const disk = mockDisk();
    disk.readFileSync.mockReturnValue(
      JSON.stringify({ workspaces: { packages: [1, 2] } }),
    );
    expect(getYarnPatterns(disk)()).toEqual(expect.any(Err));
  });
});

it('returns an new Err when disk throws', () => {
  const disk = mockDisk();
  disk.readFileSync.mockImplementation(() => {
    throw new BaseError('Failed to read JSON file');
  });
  expect(getYarnPatterns(disk)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is not valid JSON', () => {
  const disk = mockDisk();
  disk.readFileSync.mockReturnValue('wut?');
  expect(getYarnPatterns(disk)()).toEqual(expect.any(Err));
});
