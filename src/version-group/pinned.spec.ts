import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches.js';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges.js';
import { lint } from '../bin-lint/lint.js';
import { listMismatches } from '../bin-list-mismatches/list-mismatches.js';
import { list } from '../bin-list/list.js';

describe('matches', () => {
  describe('when pure semver', () => {
    describe('when all versions are identical', () => {
      describe('when no versions have a semver group', () => {
        const getScenario = createScenario({
          '.syncpackrc': {
            versionGroups: [
              {
                dependencies: ['foo'],
                packages: ['**'],
                pinVersion: '0.1.0',
              },
            ],
          },
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: '0.1.0',
            },
          },
        });

        describe('version group report', () => {
          it('is valid because they match the pinned version', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(2);
            expect(reports).toHaveProperty('0.name', 'foo');
            expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
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
            expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });

      describe('when versions have semver groups which differ from the pinned version', () => {
        const getScenario = createScenario({
          '.syncpackrc': {
            semverGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['**'],
                packages: ['**'],
                range: '^',
              },
            ],
            versionGroups: [
              {
                dependencies: ['foo'],
                packages: ['**'],
                pinVersion: '~0.1.0',
              },
            ],
          },
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: '~0.1.0',
            },
          },
        });

        describe('version group report', () => {
          it('is valid because the pinned version takes precendence', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(2);
            expect(reports).toHaveProperty('0.name', 'foo');
            expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
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
            expect(filesByName).toHaveProperty('a.dependencies.foo', '~0.1.0');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });
});

describe('mismatches', () => {
  describe('when pinVersion is pure semver', () => {
    describe('when versions are all pure semver', () => {
      describe('when only the semver range differs from the pinVersion', () => {
        describe('when no versions have a semver group', () => {
          const getScenario = createScenario({
            '.syncpackrc': {
              versionGroups: [
                {
                  dependencies: ['foo'],
                  packages: ['**'],
                  pinVersion: '~0.1.0',
                },
              ],
            },
            'package.json': {
              name: 'a',
              version: '0.0.0',
              dependencies: {
                foo: '0.1.0',
              },
            },
          });

          describe('version group report', () => {
            it('is invalid because they do not match the pinned version', async () => {
              const reports = await getScenario().getVersionReports();
              expect(reports).toHaveLength(2);
              expect(reports).toHaveProperty('0.name', 'foo');
              expect(reports).toHaveProperty('0.reports.0._tag', 'PinnedMismatch');
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
            it('fixes them to match the pinned version', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(fixMismatches(scenario));
              const filesByName = scenario.readPackages();
              expect(filesByName).toHaveProperty('a.dependencies.foo', '~0.1.0');
              expect(scenario.io.process.exit).not.toHaveBeenCalled();
            });
          });
        });

        describe('when versions have a semver group', () => {
          const getScenario = createScenario({
            '.syncpackrc': {
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  range: '',
                },
              ],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  pinVersion: '~0.1.0',
                },
              ],
            },
            'package.json': {
              name: 'a',
              version: '0.0.0',
              dependencies: {
                foo: '0.1.0',
              },
            },
          });

          describe('version group report', () => {
            it('is invalid because they do not match the pinned version which takes precedence', async () => {
              const reports = await getScenario().getVersionReports();
              expect(reports).toHaveLength(2);
              expect(reports).toHaveProperty('1.name', 'foo');
              expect(reports).toHaveProperty('1.reports.0._tag', 'PinnedMismatch');
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
            it('fixes them to match the pinned version', async () => {
              const scenario = getScenario();
              await Effect.runPromiseExit(fixMismatches(scenario));
              const filesByName = scenario.readPackages();
              expect(filesByName).toHaveProperty('a.dependencies.foo', '~0.1.0');
              expect(scenario.io.process.exit).not.toHaveBeenCalled();
            });
          });
        });
      });
    });

    describe('when versions are not semver but contain a semver substring', () => {
      describe('when the semver substring is identical to the pinVersion', () => {
        const getScenario = createScenario({
          '.syncpackrc': {
            versionGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                pinVersion: '0.1.0',
              },
            ],
          },
          'package.json': {
            name: 'a',
            version: '0.0.0',
            dependencies: {
              foo: 'git://github.com/user/project.git#0.1.0',
            },
          },
        });

        describe('version group report', () => {
          it('is invalid because they must be completely identical to the pinned version', async () => {
            const reports = await getScenario().getVersionReports();
            expect(reports).toHaveLength(2);
            expect(reports).toHaveProperty('1.name', 'foo');
            expect(reports).toHaveProperty('1.reports.0._tag', 'PinnedMismatch');
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
          it('fixes them to match the pinned version', async () => {
            const scenario = getScenario();
            await Effect.runPromiseExit(fixMismatches(scenario));
            const filesByName = scenario.readPackages();
            expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
            expect(scenario.io.process.exit).not.toHaveBeenCalled();
          });
        });
      });
    });
  });
});
