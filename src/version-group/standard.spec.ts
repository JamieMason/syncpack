import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches.js';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges.js';
import { lint } from '../bin-lint/lint.js';
import { listMismatches } from '../bin-list-mismatches/list-mismatches.js';
import { list } from '../bin-list/list.js';

describe('the "local" dependency type', () => {
  describe('when local package is missing a version property', () => {
    describe('when local package IS depended on', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          dependencyTypes: ['local', 'prod'],
        },
        'package.json': {
          name: 'foo',
        },
        'packages/a/package.json': {
          name: 'a',
          dependencies: {
            foo: '0.2.0',
          },
        },
      });

      describe('version report', () => {
        it('is broken and unfixable', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(2);
          expect(reports).toHaveProperty('1.name', 'foo');
          expect(reports).toHaveProperty('1.reports.0._tag', 'MissingLocalVersion');
          expect(reports).toHaveProperty('1.reports.1._tag', 'MissingLocalVersion');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });

    describe('when local package is NOT depended on', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          dependencyTypes: ['local', 'prod'],
        },
        'package.json': {
          name: 'foo',
        },
        'packages/a/package.json': {
          name: 'a',
        },
      });

      describe('version report', () => {
        it('is valid', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(2);
          expect(reports).toHaveProperty('1.name', 'foo');
          expect(reports).toHaveProperty('1.reports.0._tag', 'Valid');
        });
      });

      describe('lint', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list-mismatches', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('fix-mismatches', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });
  });

  describe('when local package has a non exact semver version property', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        dependencyTypes: ['local', 'prod'],
      },
      'package.json': {
        name: 'foo',
        version: '~0.2.0',
      },
      'packages/a/package.json': {
        name: 'a',
        dependencies: {
          foo: '~0.2.0',
        },
      },
    });

    describe('version report', () => {
      it('is broken and unfixable', async () => {
        const reports = await getScenario().getVersionReports();
        expect(reports).toHaveLength(2);
        expect(reports).toHaveProperty('1.name', 'foo');
        expect(reports).toHaveProperty('1.reports.0._tag', 'MissingLocalVersion');
        expect(reports).toHaveProperty('1.reports.0.unfixable.rawSpecifier.raw', '~0.2.0');
        expect(reports).toHaveProperty('1.reports.1._tag', 'MissingLocalVersion');
        expect(reports).toHaveProperty('1.reports.1.unfixable.rawSpecifier.raw', '~0.2.0');
      });
    });

    describe('lint', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(lint(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('lintSemverRanges', () => {
      it('exits 0', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(lintSemverRanges(scenario));
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('list', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(list(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('list-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(listMismatches(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when version is used which is higher than the local package', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        dependencyTypes: ['local', 'prod'],
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
      'packages/a/package.json': {
        name: 'a',
        version: '0.0.0',
        dependencies: {
          foo: '0.2.0',
        },
      },
    });

    describe('version report', () => {
      it('is invalid because the higher version must be identical to the locally developed package', async () => {
        const reports = await getScenario().getVersionReports();
        expect(reports).toHaveLength(2);
        expect(reports).toHaveProperty('1.name', 'foo');
        expect(reports).toHaveProperty('1.reports.0._tag', 'Valid');
        expect(reports).toHaveProperty('1.reports.1._tag', 'LocalPackageMismatch');
      });
    });

    describe('lint', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(lint(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('lintSemverRanges', () => {
      it('exits 0', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(lintSemverRanges(scenario));
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('list', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(list(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('list-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(listMismatches(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('fix-mismatches', () => {
      it('fixes them to match the locally developed package', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('foo.version', '0.1.0');
        expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });

  describe('when local package is referenced with "workspace:*"', () => {
    describe('with default config', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          dependencyTypes: ['local', 'prod'],
        },
        'package.json': {
          name: 'foo',
          version: '0.1.0',
        },
        'packages/a/package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: 'workspace:*',
          },
        },
      });

      describe('version report', () => {
        it('is invalid it must be identical to the locally developed package', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(2);
          expect(reports).toHaveProperty('1.name', 'foo');
          expect(reports).toHaveProperty('1.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('1.reports.1._tag', 'LocalPackageMismatch');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('fixes them to match the locally developed package', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('foo.version', '0.1.0');
          expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when configured to use "workspace:*" for local packages', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          dependencyTypes: ['local', 'dev'],
          versionGroups: [
            {
              dependencies: ['foo'],
              packages: ['**'],
              dependencyTypes: ['dev'],
              pinVersion: 'workspace:*',
            },
          ],
        },
        'package.json': {
          name: 'foo',
          version: '0.1.0',
        },
        'packages/a/package.json': {
          name: 'a',
          version: '0.0.0',
          devDependencies: {
            foo: 'workspace:*',
          },
        },
      });

      describe('version report', () => {
        it('is valid because the pinned version takes precedence', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.0.specifier.raw', 'workspace:*');
          expect(reports).toHaveProperty('2.name', 'foo');
          expect(reports).toHaveProperty('2.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('2.reports.0.specifier.raw', '0.1.0');
        });
      });

      describe('lint', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list-mismatches', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('fix-mismatches', () => {
        it('does not change anything', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('foo.version', '0.1.0');
          expect(filesByName).toHaveProperty('a.devDependencies.foo', 'workspace:*');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });
  });

  describe('when version is used which is the same as the local package except for the range', () => {
    describe('with default config', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          dependencyTypes: ['local', 'prod'],
        },
        'package.json': {
          name: 'foo',
          version: '0.1.0',
        },
        'packages/a/package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: '~0.1.0',
          },
        },
      });

      describe('version report', () => {
        it('is invalid because it must be identical to the locally developed package', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(2);
          expect(reports).toHaveProperty('1.name', 'foo');
          expect(reports).toHaveProperty('1.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('1.reports.1._tag', 'LocalPackageMismatch');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('fixes them to match the locally developed package', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('foo.version', '0.1.0');
          expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when the version used has a semver group it conforms to', () => {
      describe.each(['^', '~'])('when the range is "%s"', (range) => {
        const getScenario = createScenario({
          '.syncpackrc': {
            dependencyTypes: ['local', 'dev'],
            semverGroups: [
              {
                packages: ['**'],
                dependencies: ['**'],
                dependencyTypes: ['dev'],
                range: range,
              },
            ],
          },
          'package.json': {
            name: 'foo',
            version: '0.1.0',
          },
          'packages/a/package.json': {
            name: 'a',
            version: '0.0.0',
            devDependencies: {
              foo: `${range}0.1.0`,
            },
          },
        });

        describe('version report', () => {
          it('is valid because it matches the local version and semver range which is compatible with the version', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(2);
            expect(reports).toHaveProperty('1.name', 'foo');
            expect(reports).toHaveProperty('1.reports.0._tag', 'Valid');
            expect(reports).toHaveProperty('1.reports.1._tag', 'Valid');
          });
        });

        describe('lint', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list-mismatches', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('fix-mismatches', () => {
          it('does not change anything', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('foo.version', '0.1.0');
            expect(filesByName).toHaveProperty('a.devDependencies.foo', `${range}0.1.0`);
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });
});

describe('mismatches', () => {
  describe('when versions are pure semver', () => {
    describe('when a semver group applies', () => {
      describe('when the versions match but the ranges do not match the semver group', () => {
        const getScenario = createScenario({
          '.syncpackrc': {
            semverGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['**'],
                packages: ['**'],
                range: '',
              },
            ],
          },
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: '~3.5.2',
            },
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            devDependencies: {
              foo: '~3.5.2',
            },
          },
        });

        describe('version report', () => {
          it('is invalid because the range from the semver group is not used', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(3);
            expect(reports).toHaveProperty('2.name', 'foo');
            expect(reports).toHaveProperty('2.reports.0._tag', 'SemverRangeMismatch');
            expect(reports).toHaveProperty('2.reports.1._tag', 'SemverRangeMismatch');
          });
        });

        describe('semver report', () => {
          it('is invalid because the ranges do not match their groups', async () => {
            const reports = await getScenario().getSemverReports();
            expect(reports).toHaveLength(4);
            expect(reports).toHaveProperty('2.fixable.instance.name', 'foo');
            expect(reports).toHaveProperty('2._tag', 'SemverRangeMismatch');
            expect(reports).toHaveProperty('2.fixable.raw', '3.5.2');
            expect(reports).toHaveProperty('3.fixable.instance.name', 'foo');
            expect(reports).toHaveProperty('3._tag', 'SemverRangeMismatch');
            expect(reports).toHaveProperty('3.fixable.raw', '3.5.2');
          });
        });

        describe('lint', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('list', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('list-mismatches', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('fix-mismatches', () => {
          it('fixes them to have the range from the semver group', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', '3.5.2');
            expect(filesByName).toHaveProperty('b.devDependencies.foo', '3.5.2');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });

  describe('when versions are mixed semver and non-semver', () => {
    describe('when the versions match but the ranges do not match the semver group', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          semverGroups: [
            {
              range: '',
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: 'git://github.com/user/project.git#commit1',
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          devDependencies: {
            foo: '~3.5.2',
          },
        },
      });

      describe('version report', () => {
        it('is invalid because it is not supported', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('2.name', 'foo');
          expect(reports).toHaveProperty('2.reports.0._tag', 'UnsupportedMismatch');
          expect(reports).toHaveProperty('2.reports.1._tag', 'UnsupportedMismatch');
        });
      });

      describe('semver report', () => {
        it('is invalid because the semver ranges do not match their groups', async () => {
          const reports = await getScenario().getSemverReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('2.unfixable.name', 'foo');
          expect(reports).toHaveProperty('2._tag', 'UnsupportedMismatch');
          expect(reports).toHaveProperty('3.fixable.instance.name', 'foo');
          expect(reports).toHaveProperty('3._tag', 'SemverRangeMismatch');
          expect(reports).toHaveProperty('3.fixable.raw', '3.5.2');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });
  });

  describe('when versions are not semver', () => {
    describe('when versions do not contain semver substrings', () => {
      describe('with mismatching versions', () => {
        describe.each([
          {
            scenario: 'when versions are non-semver git tags',
            lower: 'git://github.com/user/project.git#commit1',
            higher: 'git://github.com/user/project.git#commit2',
          },
          {
            scenario: 'when versions are npm links',
            lower: 'yarn@link:vendor/yarn-fix1',
            higher: 'yarn@link:vendor/yarn-fix2',
          },
        ])('$scenario', ({ higher, lower }) => {
          const getScenario = createScenario({
            'package.json': {
              name: 'a',
              version: '0.0.0',
              dependencies: {
                foo: lower,
              },
            },
            'packages/b/package.json': {
              name: 'b',
              version: '0.0.0',
              devDependencies: {
                foo: higher,
              },
            },
            'packages/c/package.json': {
              name: 'c',
              version: '0.0.0',
              peerDependencies: {
                foo: lower,
              },
            },
          });

          describe('version report', () => {
            it('is invalid because it is not supported', async () => {
              const reports = await getScenario().getVersionReports();
              expect(reports).toHaveLength(4);
              expect(reports).toHaveProperty('3.name', 'foo');
              expect(reports).toHaveProperty('3.reports.0._tag', 'UnsupportedMismatch');
              expect(reports).toHaveProperty('3.reports.1._tag', 'UnsupportedMismatch');
              expect(reports).toHaveProperty('3.reports.2._tag', 'UnsupportedMismatch');
            });
          });

          describe('semver report', () => {
            it('is valid because semver ranges are disabled by default', async () => {
              const reports = await getScenario().getSemverReports();
              expect(reports).toHaveLength(6);
              expect(reports).toHaveProperty('3.instance.name', 'foo');
              expect(reports).toHaveProperty('3._tag', 'Disabled');
              expect(reports).toHaveProperty('4.instance.name', 'foo');
              expect(reports).toHaveProperty('4._tag', 'Disabled');
              expect(reports).toHaveProperty('5.instance.name', 'foo');
              expect(reports).toHaveProperty('5._tag', 'Disabled');
            });
          });

          describe('lint', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(lint(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('lintSemverRanges', () => {
            it('exits 0', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(lintSemverRanges(scenario));
              expect(scenario.io.process.exit).not.toHaveBeenCalled();
            });
          });

          describe('list', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(list(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('list-mismatches', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(listMismatches(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('fix-mismatches', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(fixMismatches(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });
        });
      });
    });
  });
});

describe('matches', () => {
  describe('when versions are pure semver', () => {
    describe.each([
      {
        scenario: 'when versions are exact',
        version: '1.2.3',
      },
      {
        scenario: 'when versions are "*"',
        version: '*',
      },
    ])('$scenario', ({ version }) => {
      const getScenario = createScenario({
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: version,
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          devDependencies: {
            foo: version,
          },
        },
        'packages/c/package.json': {
          name: 'c',
          version: '0.0.0',
          peerDependencies: {
            foo: version,
          },
        },
      });

      describe('version report', () => {
        it('is valid because they are identical', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('3.name', 'foo');
          expect(reports).toHaveProperty('3.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('3.reports.1._tag', 'Valid');
          expect(reports).toHaveProperty('3.reports.2._tag', 'Valid');
        });
      });

      describe('lint', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list-mismatches', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('fix-mismatches', () => {
        it('does not change anything', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.dependencies.foo', version);
          expect(filesByName).toHaveProperty('b.devDependencies.foo', version);
          expect(filesByName).toHaveProperty('c.peerDependencies.foo', version);
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when versions are not exact and have a range', () => {
      describe.each([
        {
          scenario: 'by default when checking semver ranges is disabled',
          rcFile: {},
        },
        {
          scenario: 'when semver ranges are explicitly ignored',
          rcFile: {
            semverGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        },
      ])('$scenario', ({ rcFile }) => {
        const getScenario = createScenario({
          '.syncpackrc': rcFile,
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: '~3.5.2',
            },
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            devDependencies: {
              foo: '~3.5.2',
            },
          },
        });

        describe('version report', () => {
          it('is valid because they are identical and the non-exact range is ignored', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(3);
            expect(reports).toHaveProperty('2.name', 'foo');
            expect(reports).toHaveProperty('2.reports.0._tag', 'Valid');
            expect(reports).toHaveProperty('2.reports.1._tag', 'Valid');
          });
        });

        describe('lint', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list-mismatches', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('fix-mismatches', () => {
          it('does not change anything', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', '~3.5.2');
            expect(filesByName).toHaveProperty('b.devDependencies.foo', '~3.5.2');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });

    describe('when every version in a sameRange group has a range which satisfies every other version', () => {
      describe('when semver ranges are ignored', () => {
        const getScenario = createScenario({
          '.syncpackrc': {
            semverGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
            versionGroups: [
              {
                dependencies: ['foo'],
                packages: ['**'],
                policy: 'sameRange',
              },
            ],
          },
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: '>2.0.0',
            },
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            devDependencies: {
              foo: '~3.0.0',
            },
          },
          'packages/c/package.json': {
            name: 'c',
            version: '0.0.0',
            peerDependencies: {
              foo: '^3.0.0',
            },
          },
        });

        describe('version report', () => {
          it('is valid because every version falls within the ranges of all of the others', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(4);
            expect(reports).toHaveProperty('0.name', 'foo');
            expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
            expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
            expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
          });
        });

        describe('lint', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list-mismatches', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('fix-mismatches', () => {
          it('leaves them unchanged and does not exit with an error code', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', '>2.0.0');
            expect(filesByName).toHaveProperty('b.devDependencies.foo', '~3.0.0');
            expect(filesByName).toHaveProperty('c.peerDependencies.foo', '^3.0.0');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });

    describe('when versions consistently snap to those used by another package', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          versionGroups: [
            {
              dependencies: ['foo'],
              packages: ['**'],
              snapTo: ['b'],
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: '2.0.0',
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          devDependencies: {
            foo: '2.0.0',
          },
        },
        'packages/c/package.json': {
          name: 'c',
          version: '0.0.0',
          peerDependencies: {
            foo: '2.0.0',
          },
        },
      });

      describe('version report', () => {
        it('is valid because every version is the same as the one used in the package "b"', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
        });
      });

      describe('lint', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list-mismatches', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('fix-mismatches', () => {
        it('leaves them unchanged and does not exit with an error code', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.dependencies.foo', '2.0.0');
          expect(filesByName).toHaveProperty('b.devDependencies.foo', '2.0.0');
          expect(filesByName).toHaveProperty('c.peerDependencies.foo', '2.0.0');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });
  });

  describe('when versions are not semver', () => {
    describe('when versions do not contain semver substrings', () => {
      describe.each([
        {
          scenario: 'when versions are non-semver git tags',
          version: 'git://github.com/user/project.git#commit2',
        },
        {
          scenario: 'when versions are npm links',
          version: 'yarn@link:vendor/yarn-fix',
        },
        {
          scenario: 'when versions are "workspace:*"',
          version: 'workspace:*',
        },
      ])('$scenario', ({ version }) => {
        const getScenario = createScenario({
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: version,
            },
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            devDependencies: {
              foo: version,
            },
          },
          'packages/c/package.json': {
            name: 'c',
            version: '0.0.0',
            peerDependencies: {
              foo: version,
            },
          },
        });

        describe('version report', () => {
          it('is valid because they are identical', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(4);
            expect(reports).toHaveProperty('3.name', 'foo');
            expect(reports).toHaveProperty('3.reports.0._tag', 'Valid');
            expect(reports).toHaveProperty('3.reports.1._tag', 'Valid');
            expect(reports).toHaveProperty('3.reports.2._tag', 'Valid');
          });
        });

        describe('lint', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list-mismatches', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('fix-mismatches', () => {
          it('does not change anything', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', version);
            expect(filesByName).toHaveProperty('b.devDependencies.foo', version);
            expect(filesByName).toHaveProperty('c.peerDependencies.foo', version);
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });
});

describe('when versions are pure semver', () => {
  describe('with mismatching versions', () => {
    describe('when all versions are of the same format', () => {
      describe.each([
        {
          scenario: 'when all versions are exact',
          lower: '0.2.0',
          higher: '0.3.0',
        },
        {
          scenario: 'when highest version is "*"',
          lower: '0.1.0',
          higher: '*',
        },
      ])('$scenario', ({ higher, lower }) => {
        const getScenario = createScenario({
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: lower,
            },
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            devDependencies: {
              foo: higher,
            },
          },
          'packages/c/package.json': {
            name: 'c',
            version: '0.0.0',
            peerDependencies: {
              foo: lower,
            },
          },
        });

        describe('version report', () => {
          it('is invalid because they do not match and the highest semver version should be used', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(4);
            expect(reports).toHaveProperty('3.name', 'foo');
            expect(reports).toHaveProperty('3.reports.0._tag', 'HighestSemverMismatch');
            expect(reports).toHaveProperty('3.reports.1._tag', 'Valid');
            expect(reports).toHaveProperty('3.reports.2._tag', 'HighestSemverMismatch');
          });
        });

        describe('lint', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('list-mismatches', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('fix-mismatches', () => {
          it('sets every dependency to use the highest semver version in use', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', higher);
            expect(filesByName).toHaveProperty('b.devDependencies.foo', higher);
            expect(filesByName).toHaveProperty('c.peerDependencies.foo', higher);
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });

      describe('when highest version is a greedier range than another', () => {
        describe('when semver ranges are ignored', () => {
          const getScenario = createScenario({
            '.syncpackrc': {
              semverGroups: [
                {
                  dependencies: ['**'],
                  dependencyTypes: ['**'],
                  packages: ['**'],
                  isIgnored: true,
                },
              ],
            },
            'package.json': {
              name: 'a',
              version: '0.0.0',
              dependencies: {
                foo: '<=0.1.0',
              },
            },
            'packages/b/package.json': {
              name: 'b',
              version: '0.0.0',
              devDependencies: {
                foo: '>=0.1.0',
              },
            },
            'packages/c/package.json': {
              name: 'c',
              version: '0.0.0',
              peerDependencies: {
                foo: '<=0.1.0',
              },
            },
          });

          describe('version report', () => {
            it('is invalid because they do not match and the highest semver version should be used', async () => {
              const reports = await getScenario().getVersionReports();
              expect(reports).toHaveLength(4);
              expect(reports).toHaveProperty('3.name', 'foo');
              expect(reports).toHaveProperty('3.reports.0._tag', 'HighestSemverMismatch');
              expect(reports).toHaveProperty('3.reports.1._tag', 'Valid');
              expect(reports).toHaveProperty('3.reports.2._tag', 'HighestSemverMismatch');
            });
          });

          describe('lint', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(lint(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('lintSemverRanges', () => {
            it('exits 0', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(lintSemverRanges(scenario));
              expect(scenario.io.process.exit).not.toHaveBeenCalled();
            });
          });

          describe('list', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(list(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('list-mismatches', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(listMismatches(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('fix-mismatches', () => {
            it('sets every dependency to use the highest semver version in use', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(fixMismatches(scenario));
              const filesByName = scenario.readPackages();
              expect(filesByName).toHaveProperty('a.dependencies.foo', '>=0.1.0');
              expect(filesByName).toHaveProperty('b.devDependencies.foo', '>=0.1.0');
              expect(filesByName).toHaveProperty('c.peerDependencies.foo', '>=0.1.0');
              expect(scenario.io.process.exit).not.toHaveBeenCalled();
            });
          });
        });
      });
    });

    describe('when the lowest semver version is preferred', () => {
      const lower = '0.1.0';
      const higher = '0.3.0';

      const getScenario = createScenario({
        '.syncpackrc': {
          versionGroups: [
            {
              dependencies: ['foo'],
              packages: ['**'],
              preferVersion: 'lowestSemver',
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: lower,
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          devDependencies: {
            foo: higher,
          },
        },
        'packages/c/package.json': {
          name: 'c',
          version: '0.0.0',
          peerDependencies: {
            foo: lower,
          },
        },
      });

      describe('version report', () => {
        it('is invalid because they do not match and the lowest semver version should be used', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'LowestSemverMismatch');
          expect(reports).toHaveProperty('0.reports.2._tag', 'Valid');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('sets every dependency to use the lowest semver version in use', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.dependencies.foo', lower);
          expect(filesByName).toHaveProperty('b.devDependencies.foo', lower);
          expect(filesByName).toHaveProperty('c.peerDependencies.foo', lower);
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when versions should snap to those used by another package', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          versionGroups: [
            {
              dependencies: ['foo'],
              packages: ['**'],
              snapTo: ['b'],
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: '1.0.0',
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          devDependencies: {
            foo: '2.0.0',
          },
        },
        'packages/c/package.json': {
          name: 'c',
          version: '0.0.0',
          peerDependencies: {
            foo: '3.0.0',
          },
        },
      });

      describe('version report', () => {
        it('is invalid because they do not match and the version used by "b" should be used', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'SnappedToMismatch');
          expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.2._tag', 'SnappedToMismatch');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('sets every dependency to use the version used by package "b"', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.dependencies.foo', '2.0.0');
          expect(filesByName).toHaveProperty('b.devDependencies.foo', '2.0.0');
          expect(filesByName).toHaveProperty('c.peerDependencies.foo', '2.0.0');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when a version in a sameRange group has a range which does not satisfy every other version', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          versionGroups: [
            {
              dependencies: ['foo'],
              packages: ['**'],
              policy: 'sameRange',
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: '<3.0.0',
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          devDependencies: {
            foo: '~3.0.0',
          },
        },
        'packages/c/package.json': {
          name: 'c',
          version: '0.0.0',
          peerDependencies: {
            foo: '^3.0.0',
          },
        },
      });

      describe('version report', () => {
        it('is invalid because one or more versions fall outside the ranges of one of the others', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'SameRangeMismatch');
          expect(reports).toHaveProperty('0.reports.1._tag', 'SameRangeMismatch');
          expect(reports).toHaveProperty('0.reports.2._tag', 'SameRangeMismatch');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('cannot guess what the developer wants and exits with 1 for an unfixable mismatch', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.dependencies.foo', '<3.0.0');
          expect(filesByName).toHaveProperty('b.devDependencies.foo', '~3.0.0');
          expect(filesByName).toHaveProperty('c.peerDependencies.foo', '^3.0.0');
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });

    describe('when the version is the same but the range differs', () => {
      describe('when the instances span multiple semver groups', () => {
        describe('when all instances have a semver group', () => {
          const getScenario = createScenario({
            '.syncpackrc': {
              semverGroups: [
                {
                  packages: ['a'],
                  range: '^',
                },
                {
                  packages: ['b'],
                  range: '~',
                },
                {
                  packages: ['c'],
                  range: '^',
                },
              ],
            },
            'package.json': {
              name: 'a',
              version: '0.0.0',
              dependencies: {
                foo: '~0.3.0',
              },
            },
            'packages/b/package.json': {
              name: 'b',
              version: '0.0.0',
              devDependencies: {
                foo: '^0.3.0',
              },
            },
            'packages/c/package.json': {
              name: 'c',
              version: '0.0.0',
              devDependencies: {
                foo: '~0.3.0',
              },
            },
          });

          describe('version report', () => {
            it('is invalid because one or more versions fall outside the ranges of one of the others', async () => {
              const reports = await getScenario().getVersionReports();
              expect(reports).toHaveLength(4);
              expect(reports).toHaveProperty('3.name', 'foo');
              expect(reports).toHaveProperty('3.reports.0._tag', 'HighestSemverMismatch');
              expect(reports).toHaveProperty('3.reports.1._tag', 'SemverRangeMismatch');
              expect(reports).toHaveProperty('3.reports.2._tag', 'HighestSemverMismatch');
            });
          });

          describe('semver report', () => {
            it('is invalid because the ranges do not match their groups', async () => {
              const reports = await getScenario().getSemverReports();
              expect(reports).toHaveLength(6);
              expect(reports).toHaveProperty('3.fixable.instance.name', 'foo');
              expect(reports).toHaveProperty('3._tag', 'SemverRangeMismatch');
              expect(reports).toHaveProperty('3.fixable.raw', '^0.3.0');
              expect(reports).toHaveProperty('4.fixable.instance.name', 'foo');
              expect(reports).toHaveProperty('4._tag', 'SemverRangeMismatch');
              expect(reports).toHaveProperty('4.fixable.raw', '~0.3.0');
              expect(reports).toHaveProperty('5.fixable.instance.name', 'foo');
              expect(reports).toHaveProperty('5._tag', 'SemverRangeMismatch');
              expect(reports).toHaveProperty('5.fixable.raw', '^0.3.0');
            });
          });

          describe('lint', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(lint(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('lintSemverRanges', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(lintSemverRanges(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('list', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(list(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('list-mismatches', () => {
            it('exits 1', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(listMismatches(scenario));
              expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
            });
          });

          describe('fix-mismatches', () => {
            it('sets them all to have the expected version but with the range from the semver group applied to it', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(fixMismatches(scenario));
              const filesByName = scenario.readPackages();
              expect(filesByName).toHaveProperty('a.dependencies.foo', '^0.3.0');
              expect(filesByName).toHaveProperty('b.devDependencies.foo', '~0.3.0');
              expect(filesByName).toHaveProperty('c.devDependencies.foo', '^0.3.0');
              expect(scenario.io.process.exit).not.toHaveBeenCalled();
            });
          });
        });
      });
    });
  });
});

describe('when versions are not semver but contain semver', () => {
  describe('with mismatching versions', () => {
    describe('when all versions are of the same format', () => {
      describe.each([
        {
          scenario: 'when highest version is of the format "npm:foo@0.3.0"',
          lower: 'npm:foo@0.1.0',
          higher: 'npm:foo@0.3.0',
        },
        {
          scenario: 'when highest version is of the format "git://github.com/user/project.git#0.3.0"',
          lower: 'git://github.com/user/project.git#0.1.0',
          higher: 'git://github.com/user/project.git#0.3.0',
        },
      ])('$scenario', ({ higher, lower }) => {
        const getScenario = createScenario({
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: lower,
            },
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            devDependencies: {
              foo: higher,
            },
          },
          'packages/c/package.json': {
            name: 'c',
            version: '0.0.0',
            peerDependencies: {
              foo: lower,
            },
          },
        });

        describe('version report', () => {
          it('is invalid because the semver substrings do not match', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(4);
            expect(reports).toHaveProperty('3.name', 'foo');
            expect(reports).toHaveProperty('3.reports.0._tag', 'HighestSemverMismatch');
            expect(reports).toHaveProperty('3.reports.1._tag', 'Valid');
            expect(reports).toHaveProperty('3.reports.2._tag', 'HighestSemverMismatch');
          });
        });

        describe('lint', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('list-mismatches', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('fix-mismatches', () => {
          it('sets every semver substring to use the highest semver version in use', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', higher);
            expect(filesByName).toHaveProperty('b.devDependencies.foo', higher);
            expect(filesByName).toHaveProperty('c.peerDependencies.foo', higher);
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });
});

describe('when a local package is depended on', () => {
  describe('when the dependents match each other, but are not identical to the local package', () => {
    describe('when packages are nested within other packages on the file system', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          source: ['package.json', 'packages/*/package.json', 'packages/*/apps/*/package.json'],
        },
        'package.json': {
          name: 'a',
          version: '1.1.1',
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          dependencies: {
            a: '2.2.2',
          },
        },
        'packages/b/apps/c/package.json': {
          name: 'c',
          version: '0.0.0',
          dependencies: {
            a: '2.2.2',
          },
        },
      });

      describe('version report', () => {
        it('is invalid because the dependents do not match the local version', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'a');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'LocalPackageMismatch');
          expect(reports).toHaveProperty('0.reports.2._tag', 'LocalPackageMismatch');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('sets the dependents to the local version, leaves the local version unchanged', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.version', '1.1.1');
          expect(filesByName).toHaveProperty('b.dependencies.a', '1.1.1');
          expect(filesByName).toHaveProperty('c.dependencies.a', '1.1.1');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when the dependents are exact versions', () => {
      const getScenario = createScenario({
        'package.json': {
          name: 'a',
          version: '1.1.1',
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          dependencies: {
            a: '2.2.2',
          },
        },
        'packages/c/package.json': {
          name: 'c',
          version: '0.0.0',
          dependencies: {
            a: '2.2.2',
          },
        },
      });

      describe('version report', () => {
        it('is invalid because the dependents do not match the local version', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'a');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'LocalPackageMismatch');
          expect(reports).toHaveProperty('0.reports.2._tag', 'LocalPackageMismatch');
        });
      });

      describe('lint', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lint(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lintSemverRanges', () => {
        it('exits 0', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(lintSemverRanges(scenario));
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(list(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list-mismatches', () => {
        it('exits 1', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(listMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('fix-mismatches', () => {
        it('sets the dependents to the local version, leaves the local version unchanged', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          const filesByName = scenario.readPackages();
          expect(filesByName).toHaveProperty('a.version', '1.1.1');
          expect(filesByName).toHaveProperty('b.dependencies.a', '1.1.1');
          expect(filesByName).toHaveProperty('c.dependencies.a', '1.1.1');
          expect(scenario.io.process.exit).not.toHaveBeenCalled();
        });
      });
    });

    describe('when the dependents are workspace:*', () => {
      describe('when a "sameRange" policy is NOT used', () => {
        const getScenario = createScenario({
          'package.json': {
            name: 'a',
            version: '1.1.1',
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            dependencies: {
              a: 'workspace:*',
            },
          },
          'packages/c/package.json': {
            name: 'c',
            version: '0.0.0',
            dependencies: {
              a: 'workspace:*',
            },
          },
        });

        describe('version report', () => {
          it('is invalid because the dependents do not match the local version', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(3);
            expect(reports).toHaveProperty('0.name', 'a');
            expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
            expect(reports).toHaveProperty('0.reports.1._tag', 'LocalPackageMismatch');
            expect(reports).toHaveProperty('0.reports.2._tag', 'LocalPackageMismatch');
          });
        });

        describe('lint', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('list-mismatches', () => {
          it('exits 1', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
          });
        });

        describe('fix-mismatches', () => {
          it('fixes the dependents and leaves the local version unchanged', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.version', '1.1.1');
            expect(filesByName).toHaveProperty('b.dependencies.a', '1.1.1');
            expect(filesByName).toHaveProperty('c.dependencies.a', '1.1.1');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });

      /** @see https://github.com/JamieMason/syncpack/issues/95 */
      describe('when a "sameRange" policy IS used', () => {
        const getScenario = createScenario({
          '.syncpackrc': {
            versionGroups: [
              {
                dependencies: ['a'],
                packages: ['**'],
                policy: 'sameRange',
              },
            ],
          },
          'package.json': {
            name: 'a',
            version: '1.1.1',
          },
          'packages/b/package.json': {
            name: 'b',
            version: '0.0.0',
            dependencies: {
              a: 'workspace:*',
            },
          },
          'packages/c/package.json': {
            name: 'c',
            version: '0.0.0',
            dependencies: {
              a: 'workspace:*',
            },
          },
        });

        describe('version report', () => {
          it('is valid because the local version is satisfied by the the dependents versions', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(3);
            expect(reports).toHaveProperty('0.name', 'a');
            expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
            expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
            expect(reports).toHaveProperty('0.reports.2._tag', 'Valid');
          });
        });

        describe('lint', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lint(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('lintSemverRanges', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(lintSemverRanges(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(list(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list-mismatches', () => {
          it('exits 0', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(listMismatches(scenario));
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('fix-mismatches', () => {
          it('leaves them unchanged and does not exit with an error code', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.version', '1.1.1');
            expect(filesByName).toHaveProperty('b.dependencies.a', 'workspace:*');
            expect(filesByName).toHaveProperty('c.dependencies.a', 'workspace:*');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });
});
