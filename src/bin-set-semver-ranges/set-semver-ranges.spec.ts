import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { setSemverRanges } from './set-semver-ranges.js';

describe('when a semver group contains mismatches', () => {
  describe('when "*" should be used"', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        semverGroups: [
          {
            dependencies: ['**'],
            dependencyTypes: ['**'],
            packages: ['**'],
            range: '*',
          },
        ],
      },
      'package.json': {
        name: 'a',
        version: '0.0.1',
        dependencies: {
          foo: '0.0.1',
        },
      },
      'packages/b/package.json': {
        name: 'b',
        version: '0.0.1',
        devDependencies: {
          foo: '0.0.1',
        },
      },
      'packages/c/package.json': {
        name: 'c',
        version: '0.0.1',
        peerDependencies: {
          foo: '0.0.1',
        },
      },
    });

    describe('set-semver-ranges', () => {
      it('fixes all dependencies to use "*", except for .version properties', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(setSemverRanges(scenario));
        const filesByName = scenario.readPackages();
        expect(filesByName).toHaveProperty('a.version', '0.0.1');
        expect(filesByName).toHaveProperty('a.dependencies.foo', '*');
        expect(filesByName).toHaveProperty('b.version', '0.0.1');
        expect(filesByName).toHaveProperty('b.devDependencies.foo', '*');
        expect(filesByName).toHaveProperty('c.version', '0.0.1');
        expect(filesByName).toHaveProperty('c.peerDependencies.foo', '*');
        expect(scenario.io.process.exit).not.toHaveBeenCalled();
      });
    });
  });
});
