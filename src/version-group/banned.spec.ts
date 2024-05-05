import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches.js';
import { lint } from '../bin-lint/lint.js';
import { listMismatches } from '../bin-list-mismatches/list-mismatches.js';
import { list } from '../bin-list/list.js';

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
      it('exits 0', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(lint(scenario));
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
      it('does not remove others', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
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

    it('is invalid because it should not be used', async () => {
      const reports = await getScenario().getVersionReports();
      expect(reports).toHaveLength(2);
      expect(reports).toHaveProperty('0.name', 'foo');
      expect(reports).toHaveProperty('0.reports.0._tag', 'Banned');
    });

    describe('lint', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(lint(scenario));
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
      it('removes them', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.readPackages()).not.toHaveProperty('a.dependencies.foo');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});
