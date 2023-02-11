import { R } from '@mobily/ts-belt';
import { normalize } from 'path';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { getPnpmPatterns } from './get-pnpm-patterns';

it('returns an R.Ok of strings when found', () => {
  const disk = mockDisk();
  disk.readYamlFileSync.mockReturnValue({ packages: ['a', 'b'] });
  expect(getPnpmPatterns(disk)()).toEqual(R.Ok(['a', 'b']));
});

it('returns an R.Error when disk throws', () => {
  const disk = mockDisk();
  const thrownError = new BaseError(
    `Failed to read YAML file at ${normalize('/fake/dir/pnpm-workspace.yaml')}`,
  );
  disk.readYamlFileSync.mockImplementation(() => {
    throw thrownError;
  });
  expect(getPnpmPatterns(disk)()).toEqual(R.Error(thrownError));
});

it('returns an R.Error when data is valid YAML but the wrong shape', () => {
  const disk = mockDisk();
  disk.readYamlFileSync.mockReturnValue({ packages: [1, 2] });
  expect(getPnpmPatterns(disk)()).toEqual(
    R.Error(new BaseError('no pnpm patterns found')),
  );
});
