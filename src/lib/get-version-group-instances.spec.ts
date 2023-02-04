import type { VersionGroup } from './get-context/get-groups';
import { getVersionGroupInstances } from './get-version-group-instances';

describe('getVersionGroupInstances', () => {
  it('returns a sorted list of every dependency in the project', () => {
    expect(
      getVersionGroupInstances({
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'bar', version: '0.5.0' },
        ],
      } as VersionGroup.Standard),
    ).toEqual([
      {
        hasMismatches: false,
        instances: [{ name: 'bar', version: '0.5.0' }],
        isBanned: false,
        isIgnored: false,
        isInvalid: false,
        name: 'bar',
        uniques: ['0.5.0'],
      },
      {
        hasMismatches: false,
        instances: [{ name: 'foo', version: '1.0.0' }],
        isBanned: false,
        isIgnored: false,
        isInvalid: false,
        name: 'foo',
        uniques: ['1.0.0'],
      },
    ]);
  });
  it('recognises mismatched dependency versions', () => {
    expect(
      getVersionGroupInstances({
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'foo', version: '1.1.0' },
        ],
      } as VersionGroup.Standard),
    ).toEqual([
      {
        hasMismatches: true,
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'foo', version: '1.1.0' },
        ],
        isBanned: false,
        isIgnored: false,
        isInvalid: true,
        name: 'foo',
        uniques: ['1.0.0', '1.1.0'],
      },
    ]);
  });
});
