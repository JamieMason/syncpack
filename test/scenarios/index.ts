import { mockPackage } from '../mock';
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
          before: mockPackage('a', { deps: ['foo@0.1.0'] }),
          after: mockPackage('a', { deps: ['foo@0.1.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['bar@0.2.0'] }),
          after: mockPackage('b'),
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
          before: mockPackage('a', { deps: ['foo@link:vendor/foo-0.1.0'] }),
          after: mockPackage('a', { deps: ['foo@0.3.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['foo@link:vendor/foo-0.2.0'] }),
          after: mockPackage('b', { deps: ['foo@0.3.0'] }),
        },
        {
          path: 'packages/c/package.json',
          before: mockPackage('c', { deps: ['foo@0.3.0'] }),
          after: mockPackage('c', { deps: ['foo@0.3.0'] }),
        },
        {
          path: 'packages/d/package.json',
          before: mockPackage('d', { deps: ['foo@0.2.0'] }),
          after: mockPackage('d', { deps: ['foo@0.3.0'] }),
        },
      ],
      {},
    );
  },
  /**
   * C is developed in this monorepo, its version is `0.0.1`
   * C's version is the single source of truth and should never be changed
   * A depends on C incorrectly and should be fixed
   * B depends on C incorrectly and should be fixed
   */
  dependentDoesNotMatchWorkspaceVersion() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['c@0.1.0'] }),
          after: mockPackage('a', { deps: ['c@0.0.1'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { devDeps: ['c@0.2.0'] }),
          after: mockPackage('b', { devDeps: ['c@0.0.1'] }),
        },
        {
          path: 'packages/c/package.json',
          before: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
          after: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
        },
      ],
      {},
    );
  },
  /**
   * Variation of `dependentDoesNotMatchWorkspaceVersion` in a nested workspace.
   *
   * C is developed in this monorepo, its version is `0.0.1`
   * C's version is the single source of truth and should never be changed
   * A and B depend on C incorrectly and should be fixed
   * A, B, and C are in nested workspaces
   *
   * @see https://github.com/goldstack/goldstack/pull/170/files#diff-7ae45ad102eab3b6d7e7896acd08c427a9b25b346470d7bc6507b6481575d519R19
   * @see https://github.com/JamieMason/syncpack/pull/74
   * @see https://github.com/JamieMason/syncpack/issues/66
   */
  dependentDoesNotMatchNestedWorkspaceVersion() {
    return createScenario(
      [
        {
          path: 'workspaces/a/packages/a/package.json',
          before: mockPackage('a', { deps: ['c@0.1.0'] }),
          after: mockPackage('a', { deps: ['c@0.0.1'] }),
        },
        {
          path: 'workspaces/b/packages/b/package.json',
          before: mockPackage('b', { devDeps: ['c@0.2.0'] }),
          after: mockPackage('b', { devDeps: ['c@0.0.1'] }),
        },
        {
          path: 'workspaces/b/packages/c/package.json',
          before: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
          after: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
        },
      ],
      {
        dev: true,
        overrides: false,
        pnpmOverrides: false,
        peer: false,
        prod: true,
        resolutions: false,
        workspace: true,
        source: [
          'package.json',
          'workspaces/*/package.json',
          'workspaces/*/packages/*/package.json',
        ],
      },
    );
  },
  /**
   * Only `dependencies` is unchecked
   * The semver range `~` should be used
   * A uses exact versions for `a`
   * A should be fixed to use `~` in all other cases
   */
  semverRangesDoNotMatchConfig() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            deps: ['a@0.1.0'],
            devDeps: ['b@0.1.0'],
            overrides: ['c@0.1.0'],
            pnpmOverrides: ['d@0.1.0'],
            peerDeps: ['e@0.1.0'],
            resolutions: ['f@0.1.0'],
          }),
          after: mockPackage('a', {
            deps: ['a@0.1.0'],
            devDeps: ['b@~0.1.0'],
            overrides: ['c@~0.1.0'],
            pnpmOverrides: ['d@~0.1.0'],
            peerDeps: ['e@~0.1.0'],
            resolutions: ['f@~0.1.0'],
          }),
        },
      ],
      {
        dev: true,
        overrides: true,
        pnpmOverrides: true,
        peer: true,
        prod: false,
        resolutions: true,
        workspace: true,
        semverRange: '~',
      },
    );
  },
  /**
   * Only `dependencies` is unchecked
   * The semver range `*` should be used
   * A uses exact versions for `a`
   * A should be fixed to use `*` in all other cases
   */
  semverRangesDoNotMatchConfigWildcard() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            deps: ['a@0.1.0'],
            devDeps: ['b@0.1.0'],
            overrides: ['c@0.1.0'],
            pnpmOverrides: ['d@0.1.0'],
            peerDeps: ['e@0.1.0'],
            resolutions: ['f@0.1.0'],
          }),
          after: mockPackage('a', {
            deps: ['a@0.1.0'],
            devDeps: ['*'],
            overrides: ['*'],
            pnpmOverrides: ['*'],
            peerDeps: ['*'],
            resolutions: ['*'],
          }),
        },
      ],
      {
        dev: true,
        overrides: true,
        pnpmOverrides: true,
        peer: true,
        prod: false,
        resolutions: true,
        workspace: true,
        semverRange: '*',
      },
    );
  },
  /**
   * A has a pnpm override of C
   * B has a pnpm override of C
   * The versions do not match
   * The highest semver version wins
   */
  dependentDoesNotMatchPnpmOverrideVersion() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { pnpmOverrides: ['c@0.1.0'] }),
          after: mockPackage('a', { pnpmOverrides: ['c@0.2.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { pnpmOverrides: ['c@0.2.0'] }),
          after: mockPackage('b', { pnpmOverrides: ['c@0.2.0'] }),
        },
      ],
      {
        dev: false,
        overrides: false,
        pnpmOverrides: true,
        peer: false,
        prod: false,
        resolutions: false,
        workspace: false,
      },
    );
  },
  /**
   * A has an npm override of C
   * B has an npm override of C
   * The versions do not match
   * The highest semver version wins
   */
  dependentDoesNotMatchNpmOverrideVersion() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { pnpmOverrides: ['c@0.1.0'] }),
          after: mockPackage('a', { pnpmOverrides: ['c@0.2.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { pnpmOverrides: ['c@0.2.0'] }),
          after: mockPackage('b', { pnpmOverrides: ['c@0.2.0'] }),
        },
      ],
      {
        dev: false,
        overrides: false,
        pnpmOverrides: true,
        peer: false,
        prod: false,
        resolutions: false,
        workspace: false,
      },
    );
  },
  /**
   * @see https://github.com/JamieMason/syncpack/issues/84#issue-1284878219
   */
  issue84Reproduction() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('@myscope/a', { deps: ['@myscope/a@1.0.0'] }),
          after: mockPackage('@myscope/a', { deps: ['@myscope/a@^1.0.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('@myscope/b', {}),
          after: mockPackage('@myscope/b', {}),
        },
      ],
      {
        dev: true,
        overrides: true,
        pnpmOverrides: true,
        peer: true,
        prod: true,
        resolutions: true,
        semverGroups: [
          {
            range: '^',
            dependencies: ['@myscope/**'],
            packages: ['**'],
          },
        ],
        semverRange: '~',
        workspace: true,
      },
    );
  },
};
