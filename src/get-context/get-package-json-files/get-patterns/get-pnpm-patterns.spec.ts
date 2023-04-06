import { Err, Ok } from 'tightrope/result';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { getPnpmPatterns } from './get-pnpm-patterns';

it('returns an new Ok of strings when found', () => {
  const disk = mockDisk();
  disk.readYamlFileSync.mockReturnValue({ packages: ['a', 'b'] });
  expect(getPnpmPatterns(disk)()).toEqual(new Ok(['a', 'b']));
});

it('returns an new Err when disk throws', () => {
  const disk = mockDisk();
  disk.readYamlFileSync.mockImplementation(() => {
    throw new BaseError('Failed to read YAML file');
  });
  expect(getPnpmPatterns(disk)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is valid YAML but the wrong shape', () => {
  const disk = mockDisk();
  disk.readYamlFileSync.mockReturnValue({ packages: [1, 2] });
  expect(getPnpmPatterns(disk)()).toEqual(expect.any(Err));
});
