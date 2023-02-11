import { R } from '@mobily/ts-belt';
import { mockDisk } from '../../../test/mock-disk';
import { BaseError } from '../../lib/error';
import { getConfig } from '../get-config';
import { getFilePaths } from './get-file-paths';

it('returns R.Error when patterns return no files', () => {
  const disk = mockDisk();
  const program = getConfig(disk, {});
  disk.globSync.mockReturnValue([]);
  const message =
    'No package.json files matched the patterns: "package.json", "packages/*/package.json"';
  expect(getFilePaths(disk, program)).toEqual(R.Error(new BaseError(message)));
});

it('returns R.Ok when patterns return files', () => {
  const disk = mockDisk();
  const program = getConfig(disk, {});
  const root = ['/fake/dir/package.json'];
  const packages = [
    '/fake/dir/packages/a/package.json',
    '/fake/dir/packages/b/package.json',
  ];
  disk.globSync.mockImplementation((pattern) => {
    if (pattern === 'package.json') return root;
    if (pattern === 'packages/*/package.json') return packages;
    throw new Error('Unexpected pattern in test');
  });
  expect(getFilePaths(disk, program)).toEqual(R.Ok([...root, ...packages]));
});
