import 'expect-more-jest';
import { wrapper } from '../../test/mock';
import { mockDisk } from '../../test/mock-disk';
import { getInput } from '../lib/get-input';
import { setSemverRanges } from './set-semver-ranges';

describe('setSemverRanges', () => {
  it('sets all versions to use the supplied range', () => {
    const disk = mockDisk();
    const aBefore = wrapper('a', ['foo@0.1.0', 'bar@2.0.0']);
    const aAfter = wrapper('a', ['foo@~0.1.0', 'bar@~2.0.0']);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    disk.globSync.mockImplementation((glob) => {
      if (glob.endsWith('packages/*/package.json')) {
        return ['packages/a/package.json'];
      }
    });
    disk.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('packages/a/package.json')) return aBefore.json;
    });
    setSemverRanges(
      getInput(disk, undefined, {
        dev: false,
        peer: false,
        prod: true,
        semverRange: '~',
      }),
      disk,
    );
    expect(disk.writeFileSync.mock.calls).toEqual([
      [expect.stringContaining('packages/a/package.json'), aAfter.json],
    ]);
    expect(log.mock.calls).toEqual([
      [
        expect.stringMatching(/✓/),
        expect.stringMatching('packages/a/package.json'),
      ],
    ]);
    log.mockRestore();
  });
});
