import 'expect-more-jest';
import { getInput } from '.';
import { mockDisk } from '../../../test/mock-disk';
import { DEFAULT_CONFIG } from '../../constants';

describe('getInput', () => {
  describe('dependencyTypes', () => {
    const disk = mockDisk();
    const prod = 'dependencies';
    const dev = 'devDependencies';
    const peer = 'peerDependencies';
    const workspace = 'workspace';
    const overrides = 'overrides';
    const resolutions = 'resolutions';

    it('enables them all if none are set', () => {
      expect(getInput(disk, {})).toHaveProperty(
        'dependencyTypes',
        expect.arrayContaining([
          prod,
          dev,
          peer,
          workspace,
          overrides,
          resolutions,
        ]),
      );
    });
    it('enables one if it is the only one set', () => {
      expect(getInput(disk, { prod: true })).toHaveProperty('dependencyTypes', [
        prod,
      ]);
      expect(getInput(disk, { dev: true })).toHaveProperty('dependencyTypes', [
        dev,
      ]);
      expect(getInput(disk, { peer: true })).toHaveProperty('dependencyTypes', [
        peer,
      ]);
      expect(getInput(disk, { workspace: true })).toHaveProperty(
        'dependencyTypes',
        [workspace],
      );
      expect(getInput(disk, { overrides: true })).toHaveProperty(
        'dependencyTypes',
        [overrides],
      );
      expect(getInput(disk, { resolutions: true })).toHaveProperty(
        'dependencyTypes',
        [resolutions],
      );
    });
    it('enables some if only those are set', () => {
      expect(
        getInput(disk, { dev: true, workspace: true, prod: true }),
      ).toHaveProperty(
        'dependencyTypes',
        expect.arrayContaining([prod, dev, workspace]),
      );
    });
  });
  describe('source', () => {
    it('uses defaults when no CLI options or config are set', () => {
      const disk = mockDisk();
      expect(getInput(disk, {})).toHaveProperty(
        'source',
        DEFAULT_CONFIG.source,
      );
    });
    it('uses value from config when no CLI options are set', () => {
      const disk = mockDisk();
      disk.readConfigFileSync.mockReturnValue({ source: ['./foo'] });
      expect(getInput(disk, {})).toHaveProperty('source', ['./foo']);
    });
    it('uses value from CLI when config and CLI options are set', () => {
      const disk = mockDisk();
      disk.readConfigFileSync.mockReturnValue({ source: ['./foo'] });
      expect(getInput(disk, { source: ['./bar'] })).toHaveProperty('source', [
        './bar',
      ]);
    });
    it('combines defaults, values from CLI options, and config', () => {
      const disk = mockDisk();
      disk.readConfigFileSync.mockReturnValue({ source: ['./foo'] });
      expect(getInput(disk, { sortAz: ['overridden'] })).toEqual(
        expect.objectContaining({
          semverRange: '',
          source: ['./foo'],
          sortAz: ['overridden'],
        }),
      );
    });
    describe('only available in config files', () => {
      it('merges semverGroups', () => {
        const disk = mockDisk();
        disk.readConfigFileSync.mockReturnValue({
          semverGroups: [
            {
              range: '~',
              dependencies: ['@alpha/*'],
              packages: ['@myrepo/library'],
            },
          ],
        });
        expect(getInput(disk, { sortAz: ['overridden'] }).semverGroups).toEqual(
          [
            expect.objectContaining({
              dependencies: ['@alpha/*'],
              packages: ['@myrepo/library'],
              range: '~',
            }),
            expect.objectContaining({
              dependencies: ['**'],
              packages: ['**'],
              range: '',
            }),
          ],
        );
      });
      it('merges versionGroups', () => {
        const disk = mockDisk();
        disk.readConfigFileSync.mockReturnValue({
          versionGroups: [
            { dependencies: ['chalk'], packages: ['foo', 'bar'] },
          ],
        });
        expect(
          getInput(disk, { sortAz: ['overridden'] }).versionGroups,
        ).toEqual([
          expect.objectContaining({
            dependencies: ['chalk'],
            packages: ['foo', 'bar'],
          }),
          expect.objectContaining({
            dependencies: ['**'],
            packages: ['**'],
          }),
        ]);
      });
    });
  });
  describe('wrappers', () => {
    describe('when --source cli options are given', () => {
      describe('for a single package.json file', () => {
        it('reads that file only', () => {
          const CWD = process.cwd();
          const filePath = `${CWD}/package.json`;
          const contents = { name: 'foo' };
          const json = '{"name":"foo"}';
          const disk = mockDisk();
          disk.globSync.mockReturnValue([filePath]);
          disk.readFileSync.mockReturnValue(json);
          expect(getInput(disk, { source: ['package.json'] })).toEqual(
            expect.objectContaining({
              wrappers: [{ filePath, contents, json }],
            }),
          );
        });
      });
      describe('for a pattern that matches nothing', () => {
        it('returns an empty array', () => {
          const disk = mockDisk();
          disk.globSync.mockReturnValue([]);
          expect(getInput(disk, { source: ['typo.json'] })).toHaveProperty(
            'wrappers',
            [],
          );
          expect(disk.readFileSync).not.toHaveBeenCalled();
        });
      });
    });
    describe('when no --source cli options are given', () => {
      it('performs a default search', () => {
        const disk = mockDisk();
        getInput(disk, {});
        expect(disk.globSync.mock.calls).toEqual([
          ['package.json'],
          ['packages/*/package.json'],
        ]);
      });
      describe('when yarn workspaces are defined', () => {
        describe('as an array', () => {
          it('resolves yarn workspace packages', () => {
            const CWD = process.cwd();
            const filePath = `${CWD}/package.json`;
            const contents = { workspaces: ['as-array/*'] };
            const json = JSON.stringify(contents);
            const disk = mockDisk();
            disk.globSync.mockReturnValue([filePath]);
            disk.readFileSync.mockReturnValue(json);
            getInput(disk, {});
            expect(disk.globSync.mock.calls).toEqual([
              ['package.json'],
              ['as-array/*/package.json'],
            ]);
          });
        });
        describe('as an object', () => {
          it('resolves yarn workspace packages', () => {
            const CWD = process.cwd();
            const filePath = `${CWD}/package.json`;
            const contents = { workspaces: { packages: ['as-object/*'] } };
            const json = JSON.stringify(contents);
            const disk = mockDisk();
            disk.globSync.mockReturnValue([filePath]);
            disk.readFileSync.mockReturnValue(json);
            getInput(disk, {});
            expect(disk.globSync.mock.calls).toEqual([
              ['package.json'],
              ['as-object/*/package.json'],
            ]);
          });
        });
      });
      describe('when yarn workspaces are not defined', () => {
        describe('when lerna.json is defined', () => {
          it('resolves lerna packages', () => {
            const CWD = process.cwd();
            const filePath = `${CWD}/package.json`;
            const contents = { name: 'foo' };
            const json = JSON.stringify(contents);
            const disk = mockDisk();
            disk.globSync.mockReturnValue([filePath]);
            disk.readFileSync.mockImplementation((filePath) => {
              if (filePath.endsWith('package.json')) return json;
              if (filePath.endsWith('lerna.json'))
                return JSON.stringify({ packages: ['lerna/*'] });
            });
            getInput(disk, {});
            expect(disk.globSync.mock.calls).toEqual([
              ['package.json'],
              ['lerna/*/package.json'],
            ]);
          });
        });
        describe('when lerna.json is not defined', () => {
          describe('when pnpm workspaces are defined', () => {
            it('resolves pnpm packages', () => {
              const CWD = process.cwd();
              const filePath = `${CWD}/package.json`;
              const disk = mockDisk();
              disk.globSync.mockReturnValue([filePath]);
              disk.readYamlFileSync.mockReturnValue({
                packages: ['./from-pnpm/*'],
              });
              getInput(disk, {});
              expect(disk.globSync.mock.calls).toEqual([
                ['package.json'],
                ['from-pnpm/*/package.json'],
              ]);
            });
          });
        });
      });
    });
  });
});
