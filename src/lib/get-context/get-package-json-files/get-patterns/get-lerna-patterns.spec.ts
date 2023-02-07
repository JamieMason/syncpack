import { R } from '@mobily/ts-belt';
import { mockDisk } from '../../../../../test/mock-disk';
import { BaseError } from '../../../error';
import { getLernaPatterns } from './get-lerna-patterns';

it('returns an R.Ok of strings when found', () => {
  const disk = mockDisk();
  disk.readFileSync.mockReturnValue(JSON.stringify({ packages: ['a', 'b'] }));
  expect(getLernaPatterns(disk)()).toEqual(R.Ok(['a', 'b']));
});

it('returns an R.Error when disk throws', () => {
  const disk = mockDisk();
  const thrownError = new BaseError(
    'Failed to read JSON file at /fake/dir/lerna.json',
  );
  disk.readFileSync.mockImplementation(() => {
    throw thrownError;
  });
  expect(getLernaPatterns(disk)()).toEqual(R.Error(thrownError));
});

it('returns an R.Error when data is not valid JSON', () => {
  const disk = mockDisk();
  const thrownError = new BaseError(
    'Failed to parse JSON file at /fake/dir/lerna.json',
  );
  disk.readFileSync.mockReturnValue('wut?');
  expect(getLernaPatterns(disk)()).toEqual(R.Error(thrownError));
});

it('returns an R.Error when data is valid JSON but the wrong shape', () => {
  const disk = mockDisk();
  disk.readFileSync.mockReturnValue(JSON.stringify({ packages: [1, 2] }));
  expect(getLernaPatterns(disk)()).toEqual(
    R.Error(new BaseError('no lerna patterns found')),
  );
});
