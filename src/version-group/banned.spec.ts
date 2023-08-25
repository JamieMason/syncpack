import { Effect } from 'effect';
import 'expect-more-jest';
import { createScenario } from '../../test/lib/create-scenario';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches';
import { lint } from '../bin-lint/lint';
import { listMismatches } from '../bin-list-mismatches/list-mismatches';
import { list } from '../bin-list/list';

describe('matches', () => {
  describe('when banned dependency is not used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            packages: ['**'],
            isBanned: true,
          },
        ],
      },
      'package.json': {
        name: 'a',
        version: '0.0.0',
        dependencies: {
          bar: '0.1.0',
        },
      },
    });

    describe('lint', () => {
      it('exits 0', () => {
        const scenario = getScenario();
        Effect.runSyncExit(lint(scenario));
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
      it('does not remove others', () => {
        const scenario = getScenario();
        Effect.runSyncExit(fixMismatches(scenario));
        expect(scenario.readPackages()).toHaveProperty('a.dependencies.bar', '0.1.0');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});

describe('mismatches', () => {
  describe('when a banned dependency is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            dependencies: ['foo'],
            packages: ['**'],
            isBanned: true,
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

    it('is invalid because it should not be used', () => {
      const reports = getScenario().getVersionReports();
      expect(reports).toHaveLength(2);
      expect(reports).toHaveProperty('0.name', 'foo');
      expect(reports).toHaveProperty('0.reports.0._tag', 'Banned');
    });

    describe('lint', () => {
      it('exits 1', () => {
        const scenario = getScenario();
        Effect.runSyncExit(lint(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('list', () => {
      it('exits 1', () => {
        const scenario = getScenario();
        Effect.runSyncExit(list(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('list-mismatches', () => {
      it('exits 1', () => {
        const scenario = getScenario();
        Effect.runSyncExit(listMismatches(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });

    describe('fix-mismatches', () => {
      it('removes them', () => {
        const scenario = getScenario();
        Effect.runSyncExit(fixMismatches(scenario));
        expect(scenario.readPackages()).not.toHaveProperty('a.dependencies.foo');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});
