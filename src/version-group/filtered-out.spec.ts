import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches.js';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges.js';
import { lint } from '../bin-lint/lint.js';
import { listMismatches } from '../bin-list-mismatches/list-mismatches.js';
import { list } from '../bin-list/list.js';

describe('matches', () => {
  describe('when filtered out dependencies have mismatches', () => {
    const getScenario = createScenario(
      {
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
      },
      {
        filter: 'nevergonna',
      },
    );

    it('is valid because it is excluded from the filter', async () => {
      const reports = await getScenario().getVersionReports();
      expect(reports).toHaveLength(3);
      expect(reports).toHaveProperty('2.name', 'foo');
      expect(reports).toHaveProperty('2.reports.0._tag', 'FilteredOut');
      expect(reports).toHaveProperty('2.reports.1._tag', 'FilteredOut');
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
        expect(filesByName).toHaveProperty('b.dependencies.foo', '0.3.0');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});
