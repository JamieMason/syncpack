import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('installs of a workspace package match each other but not the canonical source', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'c@>=1.0.0' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'c@>=1.0.0' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'c@>=1.0.0' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'c@>=1.0.0' } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'name@version',
                path: 'packageManager',
              },
            },
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'versionsByName',
                path: 'deps.custom',
              },
            },
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'version',
                path: 'deps.custom.c',
              },
            },
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { deps: ['c@>=1.0.0'] }),
              after: mockPackage('a', { deps: ['c@>=1.0.0'] }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { devDeps: ['c@>=1.0.0'] }),
              after: mockPackage('b', { devDeps: ['c@>=1.0.0'] }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
            },
          ],
          {
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as valid', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              expect.objectContaining({
                isValid: true,
                name: 'c',
                status: 'VALID',
              }),
            ],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          fixMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
          expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
            scenario.files['packages/c/package.json'].logEntryWhenUnchanged,
          ]);
        });
      });

      describe('list-mismatches', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          listMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          listCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
        });
      });
    });
  });
});
