import type { IndexedVersionGroup } from '../lib/get-input/get-instances';
import { listVersionGroups } from './list-version-groups';

describe('listVersionGroups', () => {
  it('returns a sorted list of every dependency in the project', () => {
    expect(
      listVersionGroups({
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'bar', version: '0.5.0' },
        ],
      } as IndexedVersionGroup),
    ).toEqual([
      {
        hasMismatches: false,
        instances: [{ name: 'bar', version: '0.5.0' }],
        isBanned: false,
        isIgnored: false,
        name: 'bar',
        uniques: ['0.5.0'],
      },
      {
        hasMismatches: false,
        instances: [{ name: 'foo', version: '1.0.0' }],
        isBanned: false,
        isIgnored: false,
        name: 'foo',
        uniques: ['1.0.0'],
      },
    ]);
  });
  it('recognises mismatched dependency versions', () => {
    expect(
      listVersionGroups({
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'foo', version: '1.1.0' },
        ],
      } as IndexedVersionGroup),
    ).toEqual([
      {
        hasMismatches: true,
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'foo', version: '1.1.0' },
        ],
        isBanned: false,
        isIgnored: false,
        name: 'foo',
        uniques: ['1.0.0', '1.1.0'],
      },
    ]);
  });
});
