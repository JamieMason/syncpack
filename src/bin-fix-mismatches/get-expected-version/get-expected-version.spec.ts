import { getExpectedVersion } from '.';
import type { Instance } from '../../lib/get-input/get-instances';

it('removes every dependency in the group if the group is marked as disallowed', () => {
  expect(
    getExpectedVersion(
      'foo',
      {
        isBanned: true,
        instances: [
          { name: 'foo', version: '2.0.0' },
          { name: 'foo', version: '3.0.0' },
          { name: 'foo', version: '1.0.0' },
        ] as Instance[],
      },
      { workspace: false, wrappers: [] },
    ),
  ).toEqual(undefined);
});

it('if not disallowed, applies pinned versions first', () => {
  expect(
    getExpectedVersion(
      'foo',
      { instances: [], pinVersion: '2.2.2' },
      { workspace: true, wrappers: [] },
    ),
  ).toEqual('2.2.2');
});

it('applies matching local package versions second, if --workspace is set', () => {
  expect(
    getExpectedVersion(
      'foo',
      { instances: [] },
      {
        workspace: true,
        wrappers: [
          {
            contents: { name: 'bar', version: '0.1.0' },
            filePath: '',
            json: '',
          },
          {
            contents: { name: 'foo', version: '1.2.3' },
            filePath: '',
            json: '',
          },
        ],
      },
    ),
  ).toEqual('1.2.3');
});

it('applies the highest installed version third, if --workspace is not set', () => {
  expect(
    getExpectedVersion(
      'foo',
      {
        instances: [
          { name: 'foo', version: '2.0.0' },
          { name: 'foo', version: '3.0.0' },
          { name: 'foo', version: '1.0.0' },
        ] as Instance[],
      },
      { workspace: false, wrappers: [] },
    ),
  ).toEqual('3.0.0');
});

it('returns an empty string if nothing matches', () => {
  expect(
    getExpectedVersion(
      'foo',
      { instances: [] },
      { workspace: false, wrappers: [] },
    ),
  ).toEqual('');
});
