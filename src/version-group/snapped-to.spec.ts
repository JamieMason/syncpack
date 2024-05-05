import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches.js';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges.js';
import { lint } from '../bin-lint/lint.js';
import { listMismatches } from '../bin-list-mismatches/list-mismatches.js';
import { list } from '../bin-list/list.js';

describe('matches', () => {
  describe('when versions are identical to the snapped to package', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            snapTo: ['a'],
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
      'packages/b/package.json': {
        name: 'b',
        version: '0.0.0',
        dependencies: {
          foo: '0.1.0',
        },
      },
    });

    it('is valid because they are identical', async () => {
      const reports = await getScenario().getVersionReports();
      expect(reports).toHaveLength(3);
      expect(reports).toHaveProperty('0.name', 'foo');
      expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
      expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
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
});

describe('mismatches', () => {
  describe('when the first snapped to package has the version', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            snapTo: ['a'],
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
      'packages/b/package.json': {
        name: 'b',
        version: '0.0.0',
        dependencies: {
          foo: '1.1.1',
        },
      },
    });

    describe('version group report', () => {
      it('is invalid because the version does not match the one used in "a"', async () => {
        const reports = await getScenario().getVersionReports();
        expect(reports).toHaveLength(3);
        expect(reports).toHaveProperty('0.name', 'foo');
        expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
        expect(reports).toHaveProperty('0.reports.1._tag', 'SnappedToMismatch');
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
      it('fixes them to match the snapped to version', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
        expect(filesByName).toHaveProperty('b.dependencies.foo', '0.1.0');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });

  describe('when the second snapped to package has the version', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            snapTo: ['b', 'c'],
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
      'packages/b/package.json': {
        name: 'b',
        version: '0.0.0',
        dependencies: {
          bar: '0.1.0',
        },
      },
      'packages/c/package.json': {
        name: 'c',
        version: '0.0.0',
        dependencies: {
          foo: '0.2.0',
        },
      },
    });

    describe('version group report', () => {
      it('is invalid because the version does not match the one used in "c"', async () => {
        const reports = await getScenario().getVersionReports();
        expect(reports).toHaveLength(5);
        expect(reports).toHaveProperty('0.name', 'foo');
        expect(reports).toHaveProperty('0.reports.0._tag', 'SnappedToMismatch');
        expect(reports).toHaveProperty('0.reports.0.fixable.raw', '0.2.0');
        expect(reports).toHaveProperty('0.reports.1._tag', 'Valid');
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
      it('fixes them to match the snapped to version', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.dependencies.foo', '0.2.0');
        expect(filesByName).toHaveProperty('b.dependencies.bar', '0.1.0');
        expect(filesByName).toHaveProperty('c.dependencies.foo', '0.2.0');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });

  describe('when no snapped to packages have the version', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            snapTo: ['nevergonna'],
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
      it('is invalid because the version does not match the one used in "a"', async () => {
        const reports = await getScenario().getVersionReports();
        expect(reports).toHaveLength(2);
        expect(reports).toHaveProperty('0.name', 'foo');
        expect(reports).toHaveProperty('0.reports.0._tag', 'MissingSnappedToMismatch');
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

  describe('when the snapped to package has a non-semver version', () => {
    describe('when no semver groups are configured', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          versionGroups: [
            {
              dependencies: ['foo'],
              snapTo: ['a'],
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
          dependencies: {
            foo: '1.1.1',
          },
        },
      });

      describe('version group report', () => {
        it('is invalid because it does not match the snapped to package', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'SnappedToMismatch');
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

      it('fixes them to match the snapped to version', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.dependencies.foo', 'git://github.com/user/project.git#commit1');
        expect(filesByName).toHaveProperty('b.dependencies.foo', 'git://github.com/user/project.git#commit1');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('when a semver group is configured', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          semverGroups: [
            {
              packages: ['b'],
              range: '',
            },
          ],
          versionGroups: [
            {
              dependencies: ['foo'],
              snapTo: ['a'],
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
          dependencies: {
            foo: '1.1.1',
          },
        },
      });

      describe('version group report', () => {
        it('is invalid because it does not match the snapped to package and that is incompatible with the semver group', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'UnsupportedMismatch');
        });
      });

      describe('semver report', () => {
        it('is valid because the current range before applying the version group rule matches the semver group', async () => {
          const reports = await getScenario().getSemverReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('2.specifier.instance.name', 'foo');
          expect(reports).toHaveProperty('2._tag', 'Valid');
          expect(reports).toHaveProperty('2.specifier.raw', '1.1.1');
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
        it('exits 1 as it cannot fix the non-semver version with the semver group', async () => {
          const scenario = getScenario();
          await Effect.runPromiseExit(fixMismatches(scenario));
          expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });
  });

  describe('when the version which mismatches the snapped to package has a non-semver version', () => {
    describe('when no semver groups are configured', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          versionGroups: [
            {
              dependencies: ['foo'],
              snapTo: ['a'],
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: '1.1.1',
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          dependencies: {
            foo: 'git://github.com/user/project.git#commit1',
          },
        },
      });

      describe('version group report', () => {
        it('is invalid because it does not match', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'SnappedToMismatch');
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

      it('fixes them to match the snapped to version', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.dependencies.foo', '1.1.1');
        expect(filesByName).toHaveProperty('b.dependencies.foo', '1.1.1');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('when a semver group is configured', () => {
      const getScenario = createScenario({
        '.syncpackrc': {
          semverGroups: [
            {
              packages: ['b'],
              range: '~',
            },
          ],
          versionGroups: [
            {
              dependencies: ['foo'],
              snapTo: ['a'],
            },
          ],
        },
        'package.json': {
          name: 'a',
          version: '0.0.0',
          dependencies: {
            foo: '1.1.1',
          },
        },
        'packages/b/package.json': {
          name: 'b',
          version: '0.0.0',
          dependencies: {
            foo: 'git://github.com/user/project.git#commit1',
          },
        },
      });

      describe('version group report', () => {
        it('is invalid because it does not match the snapped to package or the semver group', async () => {
          const reports = await getScenario().getVersionReports();
          expect(reports).toHaveLength(3);
          expect(reports).toHaveProperty('0.name', 'foo');
          expect(reports).toHaveProperty('0.reports.0._tag', 'Valid');
          expect(reports).toHaveProperty('0.reports.1._tag', 'SnappedToMismatch');
        });
      });

      describe('semver report', () => {
        it('is invalid because the range does not match the group', async () => {
          const reports = await getScenario().getSemverReports();
          expect(reports).toHaveLength(4);
          expect(reports).toHaveProperty('2.unfixable.name', 'foo');
          expect(reports).toHaveProperty('2._tag', 'UnsupportedMismatch');
          expect(reports).toHaveProperty('2.unfixable.rawSpecifier.raw', 'git://github.com/user/project.git#commit1');
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

      it('fixes them to match the snapped to version', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.dependencies.foo', '1.1.1');
        expect(filesByName).toHaveProperty('b.dependencies.foo', '~1.1.1');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});
