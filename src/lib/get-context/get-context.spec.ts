import 'expect-more-jest';
import { join } from 'path';
import { getContext } from '.';
import { mockDisk } from '../../../test/mock-disk';
import { CWD, DEFAULT_CONFIG } from '../../constants';

describe('getContext', () => {
  describe('dependencyTypes', () => {
    const disk = mockDisk();
    const dev = 'devDependencies';
    const overrides = 'overrides';
    const peer = 'peerDependencies';
    const pnpmOverrides = 'pnpmOverrides';
    const prod = 'dependencies';
    const resolutions = 'resolutions';
    const workspace = 'workspace';
    const allTypes = [
      dev,
      overrides,
      peer,
      pnpmOverrides,
      prod,
      resolutions,
      workspace,
    ];
    const ix = {
      dev: 'devDependencies',
      overrides: 'overrides',
      peer: 'peerDependencies',
      pnpmOverrides: 'pnpmOverrides',
      prod: 'dependencies',
      resolutions: 'resolutions',
      workspace: 'workspace',
    };

    it('includes all if none are set', () => {
      expect(getContext({}, disk)).toHaveProperty(
        'dependencyTypes',
        expect.toBeArrayIncludingOnly(allTypes),
      );
    });
    it('enables one if it is the only one set', () => {
      expect.assertions(allTypes.length);
      Object.entries(ix).forEach(([optionName, typeName]) => {
        expect(getContext({ [optionName]: true }, disk)).toHaveProperty(
          'dependencyTypes',
          expect.toBeArrayIncludingOnly([typeName]),
        );
      });
    });
    it('enables some if only those are set', () => {
      expect(
        getContext({ dev: true, prod: true, workspace: true }, disk),
      ).toHaveProperty(
        'dependencyTypes',
        expect.toBeArrayIncludingOnly([dev, prod, workspace]),
      );
    });
  });
  describe('source', () => {
    it('uses defaults when no CLI options or config are set', () => {
      const disk = mockDisk();
      expect(getContext({}, disk)).toHaveProperty(
        'source',
        DEFAULT_CONFIG.source,
      );
    });
    it('uses value from config when no CLI options are set', () => {
      const disk = mockDisk();
      disk.readConfigFileSync.mockReturnValue({ source: ['./foo'] });
      expect(getContext({}, disk)).toHaveProperty('source', ['./foo']);
    });
    it('uses value from CLI when config and CLI options are set', () => {
      const disk = mockDisk();
      disk.readConfigFileSync.mockReturnValue({ source: ['./foo'] });
      expect(getContext({ source: ['./bar'] }, disk)).toHaveProperty('source', [
        './bar',
      ]);
    });
    it('combines defaults, values from CLI options, and config', () => {
      const disk = mockDisk();
      disk.readConfigFileSync.mockReturnValue({ source: ['./foo'] });
      expect(getContext({ sortAz: ['overridden'] }, disk)).toEqual(
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
        expect(
          getContext({ sortAz: ['overridden'] }, disk).semverGroups,
        ).toEqual([
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
        ]);
      });
      it('merges versionGroups', () => {
        const disk = mockDisk();
        disk.readConfigFileSync.mockReturnValue({
          versionGroups: [
            { dependencies: ['chalk'], packages: ['foo', 'bar'] },
          ],
        });
        expect(
          getContext({ sortAz: ['overridden'] }, disk).versionGroups,
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
          const filePath = join(CWD, 'package.json');
          const contents = { name: 'foo' };
          const json = '{"name":"foo"}';
          const disk = mockDisk();
          disk.globSync.mockReturnValue([filePath]);
          disk.readFileSync.mockReturnValue(json);
          const program = getContext({ source: ['./package.json'] }, disk);
          expect(program).toEqual(
            expect.objectContaining({
              wrappers: [
                {
                  contents,
                  disk: expect.toBeNonEmptyObject(),
                  filePath,
                  json,
                  program: expect.toBeNonEmptyObject(),
                },
              ],
            }),
          );
        });
      });
      describe('for a pattern that matches nothing', () => {
        it('returns an empty array', () => {
          const disk = mockDisk();
          disk.globSync.mockReturnValue([]);
          expect(getContext({ source: ['typo.json'] }, disk)).toHaveProperty(
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
        getContext({}, disk);
        expect(disk.globSync.mock.calls).toEqual([
          ['package.json'],
          ['packages/*/package.json'],
        ]);
      });
      describe('when yarn workspaces are defined', () => {
        describe('as an array', () => {
          it('resolves yarn workspace packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { workspaces: ['./as-array/*'] };
            const json = JSON.stringify(contents);
            const disk = mockDisk();
            disk.globSync.mockReturnValue([filePath]);
            disk.readFileSync.mockReturnValue(json);
            getContext({}, disk);
            expect(disk.globSync.mock.calls).toEqual([
              ['./package.json'],
              ['./as-array/*/package.json'],
            ]);
          });
        });
        describe('as an object', () => {
          it('resolves yarn workspace packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { workspaces: { packages: ['./as-object/*'] } };
            const json = JSON.stringify(contents);
            const disk = mockDisk();
            disk.globSync.mockReturnValue([filePath]);
            disk.readFileSync.mockReturnValue(json);
            getContext({}, disk);
            expect(disk.globSync.mock.calls).toEqual([
              ['./package.json'],
              ['./as-object/*/package.json'],
            ]);
          });
        });
      });
      describe('when yarn workspaces are not defined', () => {
        describe('when lerna.json is defined', () => {
          it('resolves lerna packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { name: 'foo' };
            const json = JSON.stringify(contents);
            const disk = mockDisk();
            disk.globSync.mockReturnValue([filePath]);
            disk.readFileSync.mockImplementation((filePath) => {
              if (filePath.endsWith('package.json')) return json;
              if (filePath.endsWith('lerna.json'))
                return JSON.stringify({ packages: ['./lerna/*'] });
            });
            getContext({}, disk);
            expect(disk.globSync.mock.calls).toEqual([
              ['./package.json'],
              ['./lerna/*/package.json'],
            ]);
          });
        });
        describe('when lerna.json is not defined', () => {
          describe('when pnpm config is present', () => {
            describe('when pnpm workspaces are defined', () => {
              it('resolves pnpm packages', () => {
                const filePath = join(CWD, 'package.json');
                const disk = mockDisk();
                disk.globSync.mockReturnValue([filePath]);
                disk.readYamlFileSync.mockReturnValue({
                  packages: ['./from-pnpm/*'],
                });
                getContext({}, disk);
                expect(disk.globSync.mock.calls).toEqual([
                  ['./package.json'],
                  ['./from-pnpm/*/package.json'],
                ]);
              });
            });
            describe('when pnpm config is invalid', () => {
              it('performs a default search', () => {
                const filePath = join(CWD, 'package.json');
                const disk = mockDisk();
                disk.globSync.mockReturnValue([filePath]);
                disk.readYamlFileSync.mockImplementation(() => {
                  throw new Error('Some YAML Error');
                });
                getContext({}, disk);
                expect(disk.globSync.mock.calls).toEqual([
                  ['package.json'],
                  ['packages/*/package.json'],
                ]);
              });
            });
          });
        });
      });
    });
  });
});
