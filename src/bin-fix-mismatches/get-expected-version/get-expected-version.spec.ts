import { getExpectedVersion } from '.';
import type { Instance } from '../../lib/get-input/get-instances';

it('applies pinned versions first', () => {
  expect(
    getExpectedVersion(
      'foo',
      { instances: [], pinVersion: '2.2.2' },
      { wrappers: [] },
    ),
  ).toEqual('2.2.2');
});

it('applies matching local package versions second', () => {
  expect(
    getExpectedVersion(
      'foo',
      { instances: [] },
      {
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

it('applies the highest installed version third', () => {
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
      { wrappers: [] },
    ),
  ).toEqual('3.0.0');
});

it('returns an empty string if nothing matches', () => {
  expect(
    getExpectedVersion('foo', { instances: [] }, { wrappers: [] }),
  ).toEqual('');
});
