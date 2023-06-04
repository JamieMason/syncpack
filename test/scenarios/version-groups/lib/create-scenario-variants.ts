import type { Context } from '../../../../src/get-context';
import { splitNameAndVersion } from '../../../../src/lib/split-name-and-version';
import { mockPackage } from '../../../mock';
import { createScenario } from '../../lib/create-scenario';

export function createScenarioVariants(options: {
  config: Context['config']['rcFile'];
  a: [before: string, after: string];
  b: [before: string, after: string];
  c?: [before: string, after: string];
}) {
  const { config } = options;

  function getFiles() {
    return ['a', 'b', 'c']
      .filter((key) => options[key])
      .map((key) => {
        const [before, after] = options[key];
        const [nameBefore, versionBefore] = splitNameAndVersion(before);
        const [nameAfter, versionAfter] = splitNameAndVersion(after);
        return {
          key,
          nameBefore,
          versionBefore,
          nameAfter,
          versionAfter,
        };
      });
  }

  return [
    () =>
      createScenario(
        getFiles().map((file) => ({
          path: `packages/${file.key}/package.json`,
          before: mockPackage(file.key, {
            otherProps: {
              packageManager: `${file.nameBefore}@${file.versionBefore}`,
            },
          }),
          after: mockPackage(file.key, {
            otherProps: {
              packageManager: `${file.nameAfter}@${file.versionAfter}`,
            },
          }),
        })),
        {
          customTypes: {
            engines: { strategy: 'name@version', path: 'packageManager' },
          },
          ...config,
        },
      ),
    () =>
      createScenario(
        getFiles().map((file) => ({
          path: `packages/${file.key}/package.json`,
          before: mockPackage(file.key, {
            otherProps: {
              deps: { custom: { [file.nameBefore]: file.versionBefore } },
            },
          }),
          after: mockPackage(file.key, {
            otherProps: {
              deps: { custom: { [file.nameAfter]: file.versionAfter } },
            },
          }),
        })),
        {
          customTypes: {
            engines: { strategy: 'versionsByName', path: 'deps.custom' },
          },
          ...config,
        },
      ),
    () =>
      createScenario(
        getFiles().map((file) => ({
          path: `packages/${file.key}/package.json`,
          before: mockPackage(file.key, {
            otherProps: {
              deps: { custom: { [file.nameBefore]: file.versionBefore } },
            },
          }),
          after: mockPackage(file.key, {
            otherProps: {
              deps: { custom: { [file.nameAfter]: file.versionAfter } },
            },
          }),
        })),
        {
          customTypes: {
            engines: { strategy: 'version', path: 'deps.custom.yarn' },
          },
          ...config,
        },
      ),
    ...[
      'deps',
      'devDeps',
      'overrides',
      'peerDeps',
      'pnpmOverrides',
      'resolutions',
    ].map(
      (type: string) => () =>
        createScenario(
          getFiles().map((file) => ({
            path: `packages/${file.key}/package.json`,
            before: mockPackage(file.key, {
              [type]: [`${file.nameBefore}@${file.versionBefore}`],
            }),
            after: mockPackage(file.key, {
              [type]: [`${file.nameAfter}@${file.versionAfter}`],
            }),
          })),
          config,
        ),
    ),
  ];
}
