import { wrapper } from '../mock';
import { createScenario } from './create-scenario';

export const scenarios = {
  /**
   * A does not depend on `bar`
   * B does depend on `bar`
   * `bar` is banned in every package from being installed
   * `bar` should be removed from B
   */
  dependencyIsBanned() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: wrapper('a', ['foo@0.1.0']),
          after: wrapper('a', ['foo@0.1.0']),
        },
        {
          path: 'packages/b/package.json',
          before: wrapper('b', ['bar@0.2.0']),
          after: wrapper('b'),
        },
      ],
      {
        versionGroups: [
          {
            dependencies: ['bar'],
            packages: ['**'],
            isBanned: true,
          },
        ],
      },
    );
  },
  /**
   * A, B, C & D depend on foo
   * The versions mismatch
   * Some versions are not semver
   * `0.3.0` is the highest valid semver version
   * All packages should be fixed to use `0.3.0`
   */
  mismatchesIncludeNonSemverVersions() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: wrapper('a', ['foo@link:vendor/foo-0.1.0']),
          after: wrapper('a', ['foo@0.3.0']),
        },
        {
          path: 'packages/b/package.json',
          before: wrapper('b', ['foo@link:vendor/foo-0.2.0']),
          after: wrapper('b', ['foo@0.3.0']),
        },
        {
          path: 'packages/c/package.json',
          before: wrapper('c', ['foo@0.3.0']),
          after: wrapper('c', ['foo@0.3.0']),
        },
        {
          path: 'packages/d/package.json',
          before: wrapper('d', ['foo@0.2.0']),
          after: wrapper('d', ['foo@0.3.0']),
        },
      ],
      {},
    );
  },
  /**
   * C is developed in this monorepo, its version is `0.0.1`
   * C's version is the single source of truth and should never be changed
   * A and B depend on C incorrectly and should be fixed
   */
  dependentDoesNotMatchWorkspaceVersion() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: wrapper('a', ['c@0.1.0']),
          after: wrapper('a', ['c@0.0.1']),
        },
        {
          path: 'packages/b/package.json',
          before: wrapper('b', [], ['c@0.2.0']),
          after: wrapper('b', [], ['c@0.0.1']),
        },
        {
          path: 'packages/c/package.json',
          before: wrapper('c', [], [], [], {
            name: 'c',
            version: '0.0.1',
          }),
          after: wrapper('c', [], [], [], {
            name: 'c',
            version: '0.0.1',
          }),
        },
      ],
      {},
    );
  },
  /**
   * Variation of the previous scenario in a nested workspace.
   * 
   */
  dependentDoesNotMatchNestedWorkspaceVersion() {
    return createScenario(
      [
        {
          path: 'workspaces/a/packages/a/package.json',
          before: wrapper('a', ['c@0.1.0']),
          after: wrapper('a', ['c@0.0.1']),
        },
        {
          path: 'workspaces/b/packages/b/package.json',
          before: wrapper('b', [], ['c@0.2.0']),
          after: wrapper('b', [], ['c@0.0.1']),
        },
        {
          path: 'workspaces/b/packages/c/package.json',
          before: wrapper('c', [], [], [], {
            name: 'c',
            version: '0.0.1',
          }),
          after: wrapper('c', [], [], [], {
            name: 'c',
            version: '0.0.1',
          }),
        },
      ],
      {},
    );
  },
  /**
   * Only `dependencies` are checked
   * The semver range `~` should be used
   * A uses exact versions for `foo` and `bar`
   * A should be fixed to use `~` in both cases
   */
  semverRangesDoNotMatchConfig() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: wrapper('a', ['foo@0.1.0', 'bar@2.0.0']),
          after: wrapper('a', ['foo@~0.1.0', 'bar@~2.0.0']),
        },
      ],
      {
        dev: false,
        peer: false,
        prod: true,
        semverRange: '~',
      },
    );
  },
};
