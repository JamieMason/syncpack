import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has multiple custom types defined to run on every package
 * - Each semver group applies to one custom type
 * - All of the semver groups should run and fix
 */
export function customTypesAndSemverGroups() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: {
            packageManager: 'yarn@~2.0.0',
            engines: {
              node: '16.16.0',
              npm: '7.24.2',
            },
          },
        }),
        after: mockPackage('a', {
          otherProps: {
            packageManager: 'yarn@2.0.0',
            engines: {
              node: '>=16.16.0',
              npm: '^7.24.2',
            },
          },
        }),
      },
    ],
    {
      customTypes: {
        enginesNpm: {
          path: 'engines.npm',
          strategy: 'version',
        },
        enginesNode: {
          path: 'engines.node',
          strategy: 'version',
        },
        packageManager: {
          path: 'packageManager',
          strategy: 'name@version',
        },
      },
      semverGroups: [
        {
          dependencyTypes: ['enginesNode'],
          dependencies: ['**'],
          packages: ['**'],
          range: '>=',
        },
        {
          dependencyTypes: ['enginesNpm'],
          dependencies: ['**'],
          packages: ['**'],
          range: '^',
        },
        {
          dependencyTypes: ['packageManager'],
          dependencies: ['**'],
          packages: ['**'],
          range: '',
        },
      ],
    },
  );
}
