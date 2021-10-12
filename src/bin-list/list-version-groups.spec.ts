import { listVersionGroups } from './list-version-groups';

describe('listVersionGroups', () => {
  it('returns a sorted list of every dependency in the project', () => {
    expect(
      listVersionGroups([
        { name: 'foo', version: '1.0.0' },
        { name: 'bar', version: '0.5.0' },
      ] as any),
    ).toEqual([
      {
        hasMismatches: false,
        instances: [{ name: 'bar', version: '0.5.0' }],
        name: 'bar',
        uniques: ['0.5.0'],
      },
      {
        hasMismatches: false,
        instances: [{ name: 'foo', version: '1.0.0' }],
        name: 'foo',
        uniques: ['1.0.0'],
      },
    ]);
  });
  it('recognises mismatched dependency versions', () => {
    expect(
      listVersionGroups([
        { name: 'foo', version: '1.0.0' },
        { name: 'foo', version: '1.1.0' },
      ] as any),
    ).toEqual([
      {
        hasMismatches: true,
        instances: [
          { name: 'foo', version: '1.0.0' },
          { name: 'foo', version: '1.1.0' },
        ],
        name: 'foo',
        uniques: ['1.0.0', '1.1.0'],
      },
    ]);
  });
});
