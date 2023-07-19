import * as Effect from '@effect/io/Effect';
import util from 'node:util';
import { lintSemverRanges } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { setSemverRanges } from '../../../src/bin-set-semver-ranges/set-semver-ranges';
import { toBeValid, toBeWorkspaceSemverRangeMismatch } from '../../matchers/semver-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('Issue 121', () => {
  it('should identify as a workspace match', () => {
    const scenario = createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['foo@workspace:*'] }),
          after: mockPackage('a', { deps: ['foo@workspace:*'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['foo@workspace:*'] }),
          after: mockPackage('b', { deps: ['foo@workspace:*'] }),
        },
      ],
      { cli: {}, rcFile: {} },
    );

    console.info(util.inspect(scenario.report, { depth: null }));

    expect(scenario.report.versionGroups).toEqual([[toBeValid({ name: 'foo' })]]);
  });
});
