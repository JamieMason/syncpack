import { Effect } from 'effect';
import 'expect-more-jest';
import { createScenario } from '../../test/lib/create-scenario';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges';
import { lint } from '../bin-lint/lint';
import { listMismatches } from '../bin-list-mismatches/list-mismatches';
import { list } from '../bin-list/list';

describe('matches', () => {
  describe('when ignored dependencies have mismatches', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            packages: ['**'],
            isIgnored: true,
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
          foo: '0.3.0',
        },
      },
    });

    it('is valid because it is excluded from the filter', () => {
      const reports = getScenario().getVersionReports();
      expect(reports).toHaveLength(3);
      expect(reports).toHaveProperty('0.name', 'foo');
      expect(reports).toHaveProperty('0.reports.0._tag', 'Ignored');
      expect(reports).toHaveProperty('0.reports.1._tag', 'Ignored');
    });

    describe('lint', () => {
      it('exits 0', () => {
        const scenario = getScenario();
        Effect.runSyncExit(lint(scenario));
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('lintSemverRanges', () => {
      it('exits 0', () => {
        const scenario = getScenario();
        Effect.runSyncExit(lintSemverRanges(scenario));
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('list', () => {
      it('exits 0', () => {
        const scenario = getScenario();
        Effect.runSyncExit(list(scenario));
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('list-mismatches', () => {
      it('exits 0', () => {
        const scenario = getScenario();
        Effect.runSyncExit(listMismatches(scenario));
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });

    describe('fix-mismatches', () => {
      it('does not change anything', () => {
        const scenario = getScenario();
        Effect.runSyncExit(fixMismatches(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.dependencies.foo', '0.1.0');
        expect(filesByName).toHaveProperty('b.dependencies.foo', '0.3.0');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});
