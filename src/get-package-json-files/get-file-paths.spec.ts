import { Err, Ok } from 'tightrope/result';
import { mockDisk } from '../../test/mock-disk';
import { getContext } from '../get-context';
import { getFilePaths } from './get-file-paths';

it('returns new Err when patterns return no files', () => {
  const disk = mockDisk();
  const { config } = getContext({}, disk);
  disk.globSync.mockReturnValue([]);
  const message = 'No files matched "package.json", "packages/*/package.json"';
  expect(getFilePaths(disk, config)).toEqual(new Err(new Error(message)));
});

it('returns new Ok when patterns return files', () => {
  const disk = mockDisk();
  const { config } = getContext({}, disk);
  const root = ['/fake/dir/package.json'];
  const packages = ['/fake/dir/packages/a/package.json', '/fake/dir/packages/b/package.json'];
  disk.globSync.mockImplementation((pattern) => {
    if (pattern === 'package.json') return root;
    if (pattern === 'packages/*/package.json') return packages;
    throw new Error('Unexpected pattern in test');
  });
  expect(getFilePaths(disk, config)).toEqual(new Ok([...root, ...packages]));
});
