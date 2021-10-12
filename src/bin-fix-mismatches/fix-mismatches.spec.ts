import 'expect-more-jest';
import { wrapper } from '../../test/mock';
import { mockDisk } from '../../test/mock-disk';
import { getInput } from '../lib/get-input';
import { fixMismatches } from './fix-mismatches';

describe('fixMismatches', () => {
  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is not a package maintained in this workspace', () => {
      it('uses the highest version', () => {
        const disk = mockDisk();
        const aBefore = wrapper('a', ['foo@0.1.0']);
        const bBefore = wrapper('b', [], ['foo@0.2.0']);
        const aAfter = wrapper('a', ['foo@0.2.0']);
        const log = jest
          .spyOn(console, 'log')
          .mockImplementation(() => undefined);
        disk.globSync.mockImplementation((glob) => {
          if (glob.endsWith('packages/*/package.json')) {
            return ['packages/a/package.json', 'packages/b/package.json'];
          }
        });
        disk.readFileSync.mockImplementation((filePath) => {
          if (filePath.endsWith('packages/a/package.json')) {
            return aBefore.json;
          }
          if (filePath.endsWith('packages/b/package.json')) {
            return bBefore.json;
          }
        });
        fixMismatches(getInput(disk, {}), disk);
        expect(disk.writeFileSync.mock.calls).toEqual([
          [expect.stringContaining('packages/a/package.json'), aAfter.json],
        ]);
        expect(log.mock.calls).toEqual([
          [
            expect.stringMatching(/✓/),
            expect.stringMatching('packages/a/package.json'),
          ],
          [
            expect.stringMatching(/-/),
            expect.stringMatching('packages/b/package.json'),
          ],
        ]);
        log.mockRestore();
      });
    });

    describe('when the dependency is a package maintained in this workspace', () => {
      it('uses the workspace version', () => {
        const disk = mockDisk();
        const aBefore = wrapper('a', ['foo@0.1.0']);
        const bBefore = wrapper('b', [], ['foo@0.2.0']);
        const fooBefore = wrapper('foo', [], [], [], {
          name: 'foo',
          version: '0.0.1',
        });
        const aAfter = wrapper('a', ['foo@0.0.1']);
        const bAfter = wrapper('b', [], ['foo@0.0.1']);
        const log = jest
          .spyOn(console, 'log')
          .mockImplementation(() => undefined);
        disk.globSync.mockImplementation((glob) => {
          if (glob.endsWith('packages/*/package.json')) {
            return [
              'packages/a/package.json',
              'packages/b/package.json',
              'packages/foo/package.json',
            ];
          }
        });
        disk.readFileSync.mockImplementation((filePath) => {
          if (filePath.endsWith('packages/a/package.json')) return aBefore.json;
          if (filePath.endsWith('packages/b/package.json')) return bBefore.json;
          if (filePath.endsWith('packages/foo/package.json'))
            return fooBefore.json;
        });
        fixMismatches(getInput(disk, {}), disk);
        expect(disk.writeFileSync.mock.calls).toEqual([
          [expect.stringContaining('packages/a/package.json'), aAfter.json],
          [expect.stringContaining('packages/b/package.json'), bAfter.json],
        ]);
        expect(log.mock.calls).toEqual([
          [
            expect.stringMatching(/✓/),
            expect.stringMatching('packages/a/package.json'),
          ],
          [
            expect.stringMatching(/✓/),
            expect.stringMatching('packages/b/package.json'),
          ],
          [
            expect.stringMatching(/-/),
            expect.stringMatching('packages/foo/package.json'),
          ],
        ]);
        log.mockRestore();
      });
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const disk = mockDisk();
      const aBefore = wrapper('a', ['foo@link:vendor/foo-0.1.0']);
      const bBefore = wrapper('b', ['foo@link:vendor/foo-0.2.0']);
      const cBefore = wrapper('c', ['foo@0.3.0']);
      const dBefore = wrapper('d', ['foo@0.2.0']);
      const aAfter = wrapper('a', ['foo@0.3.0']);
      const bAfter = wrapper('b', ['foo@0.3.0']);
      const dAfter = wrapper('d', ['foo@0.3.0']);
      const log = jest
        .spyOn(console, 'log')
        .mockImplementation(() => undefined);
      disk.globSync.mockImplementation((glob) => {
        if (glob.endsWith('packages/*/package.json')) {
          return [
            'packages/a/package.json',
            'packages/b/package.json',
            'packages/c/package.json',
            'packages/d/package.json',
          ];
        }
      });
      disk.readFileSync.mockImplementation((filePath) => {
        if (filePath.endsWith('packages/a/package.json')) return aBefore.json;
        if (filePath.endsWith('packages/b/package.json')) return bBefore.json;
        if (filePath.endsWith('packages/c/package.json')) return cBefore.json;
        if (filePath.endsWith('packages/d/package.json')) return dBefore.json;
      });
      fixMismatches(getInput(disk, {}), disk);
      expect(disk.writeFileSync.mock.calls).toEqual([
        [expect.stringContaining('packages/a/package.json'), aAfter.json],
        [expect.stringContaining('packages/b/package.json'), bAfter.json],
        [expect.stringContaining('packages/d/package.json'), dAfter.json],
      ]);
      expect(log.mock.calls).toEqual([
        [
          expect.stringMatching(/✓/),
          expect.stringMatching('packages/a/package.json'),
        ],
        [
          expect.stringMatching(/✓/),
          expect.stringMatching('packages/b/package.json'),
        ],
        [
          expect.stringMatching(/-/),
          expect.stringMatching('packages/c/package.json'),
        ],
        [
          expect.stringMatching(/✓/),
          expect.stringMatching('packages/d/package.json'),
        ],
      ]);
      log.mockRestore();
    });
  });
});
