import { Err, Ok } from 'tightrope/result';
import { mockDisk } from '../../../test/mock-disk';
import { getLernaPatterns } from './get-lerna-patterns';

it('returns an new Ok of strings when found', () => {
  const disk = mockDisk();
  disk.readFileSync.mockReturnValue(JSON.stringify({ packages: ['a', 'b'] }));
  expect(getLernaPatterns(disk)()).toEqual(new Ok(['a', 'b']));
});

it('returns an new Err when disk throws', () => {
  const disk = mockDisk();
  disk.readFileSync.mockImplementation(() => {
    throw new Error('Failed to read JSON file');
  });
  expect(getLernaPatterns(disk)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is not valid JSON', () => {
  const disk = mockDisk();
  disk.readFileSync.mockReturnValue('wut?');
  expect(getLernaPatterns(disk)()).toEqual(expect.any(Err));
});

it('returns an new Err when data is valid JSON but the wrong shape', () => {
  const disk = mockDisk();
  disk.readFileSync.mockReturnValue(JSON.stringify({ packages: [1, 2] }));
  expect(getLernaPatterns(disk)()).toEqual(expect.any(Err));
});
