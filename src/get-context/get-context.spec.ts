import { expect } from '@jest/globals';
import { Effect, pipe } from 'effect';
import 'expect-more-jest';
import { normalize } from 'path';
import { getContext } from '.';
import { createScenario } from '../../test/lib/create-scenario';
import { shape } from '../../test/lib/matchers';
import { DEFAULT_CONFIG } from '../constants';
import { NoSourcesFoundError } from '../get-package-json-files/get-file-paths';
import { JsonParseError } from '../io/read-json-file-sync';

it('errors when no package.json is found', () => {
  const scenario = createScenario({})();
  Effect.runSyncExit(pipe(getContext(scenario), Effect.merge));
  expect(scenario.errorHandlers.NoSourcesFoundError).toHaveBeenCalledWith(
    new NoSourcesFoundError({
      CWD: '/fake/dir',
      patterns: [...DEFAULT_CONFIG.source],
    }),
  );
});

it('errors when package.json is invalid', () => {
  const scenario = createScenario({
    'package.json': 'THIS-IS-NOT-VALID-JSON',
    'packages/bar/package.json': {
      name: 'bar',
    },
  })();
  Effect.runSyncExit(pipe(getContext(scenario), Effect.merge));
  expect(scenario.errorHandlers.JsonParseError).toHaveBeenCalledWith(
    new JsonParseError({
      error: expect.any(SyntaxError),
      filePath: expect.stringContaining('/package.json') as unknown as string,
      json: 'THIS-IS-NOT-VALID-JSON',
    }),
  );
});

it('uses empty config and default sources when no user config is found', () => {
  const scenario = createScenario({
    'package.json': {
      name: 'foo',
    },
    'packages/bar/package.json': {
      name: 'bar',
    },
  })();
  const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
  expect(ctx).toHaveProperty('config', {
    cli: scenario.cli,
    rcFile: {},
  });
  expect(ctx).toHaveProperty('packageJsonFiles', [
    shape.PackageJsonFile({
      shortPath: 'package.json',
    }),
    shape.PackageJsonFile({
      shortPath: normalize('packages/bar/package.json'),
    }),
  ]);
  expect(ctx).toEqual(
    expect.objectContaining({
      packageJsonFilesByName: expect.any(Object),
    }),
  );
});

describe('finds package.json files', () => {
  test('in .syncpackrc', () => {
    const scenario = createScenario({
      '.syncpackrc': {
        source: ['apps/**'],
      },
      'apps/bar/package.json': {
        name: 'bar',
      },
      'package.json': {
        name: 'foo',
      },
    })();
    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {
        source: ['apps/**'],
      },
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: normalize('apps/bar/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in --source options', () => {
    const scenario = createScenario(
      {
        '.syncpackrc': {
          source: ['apps/**'],
        },
        'apps/bar/package.json': {
          name: 'bar',
        },
        'apps/baz/package.json': {
          name: 'baz',
        },
        'package.json': {
          name: 'foo',
        },
      },
      {
        source: ['apps/baz/package.json'],
      },
    )();
    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {
        source: ['apps/**'],
      },
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: normalize('apps/baz/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in `.workspaces.packages` for yarn', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
        workspaces: {
          packages: ['apps/**'],
        },
      },
      'apps/bar/package.json': {
        name: 'bar',
      },
    })();
    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {},
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
      shape.PackageJsonFile({
        shortPath: normalize('apps/bar/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in `.workspaces` for yarn', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
        workspaces: ['apps/**'],
      },
      'apps/bar/package.json': {
        name: 'bar',
      },
    })();

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {},
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
      shape.PackageJsonFile({
        shortPath: normalize('apps/bar/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in pnpm-workspace.yaml', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
      },
      'apps/bar/package.json': {
        name: 'bar',
      },
      'pnpm-workspace.yaml': {
        packages: ['apps/**'],
      },
    })();

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {},
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
      shape.PackageJsonFile({
        shortPath: normalize('apps/bar/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in lerna.json when pnpm-workspace.yaml is invalid', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
      },
      'apps/bar/package.json': {
        name: 'bar',
      },
      'components/baz/package.json': {
        name: 'baz',
      },
      'lerna.json': {
        packages: ['components/**'],
      },
      'pnpm-workspace.yaml': {
        packages: ['apps/**'],
      },
    })();
    const spy = jest.spyOn(scenario.io.readYamlFile, 'sync');
    spy.mockImplementation(() => {
      throw new Error('some error');
    });

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {},
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
      shape.PackageJsonFile({
        shortPath: normalize('components/baz/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in defaults when lerna.json is invalid', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
      },
      'packages/bar/package.json': {
        name: 'bar',
      },
      'lerna.json': 'NOT-VALID-JSON',
    })();

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {},
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
      shape.PackageJsonFile({
        shortPath: normalize('packages/bar/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in defaults when lerna.json does not have the required data', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
      },
      'packages/bar/package.json': {
        name: 'bar',
      },
      'lerna.json': {
        something: 'else',
      },
    })();

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {},
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
      shape.PackageJsonFile({
        shortPath: normalize('packages/bar/package.json'),
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });
});

describe('finds syncpack config file', () => {
  test('in .syncpackrc', () => {
    const scenario = createScenario({
      '.syncpackrc': {
        dependencyTypes: ['prod'],
      },
      'package.json': {
        name: 'foo',
      },
    })();

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {
        dependencyTypes: ['prod'],
      },
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('when alternative configPath is given', () => {
    const scenario = createScenario(
      {
        '.foorc': {
          dependencyTypes: ['dev'],
        },
        '.syncpackrc': {
          dependencyTypes: ['prod'],
        },
        'package.json': {
          name: 'foo',
        },
      },
      {
        configPath: '.foorc',
      },
    )();
    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {
        dependencyTypes: ['dev'],
      },
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
    ]);
    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });

  test('in package.json when no .syncpackrc is present', () => {
    const scenario = createScenario({
      'package.json': {
        name: 'foo',
        config: {
          syncpack: {
            dependencyTypes: ['peer'],
          },
        },
      },
    })();

    const ctx = Effect.runSync(pipe(getContext(scenario), Effect.merge));
    expect(ctx).toHaveProperty('config', {
      cli: scenario.cli,
      rcFile: {
        dependencyTypes: ['peer'],
      },
    });
    expect(ctx).toHaveProperty('packageJsonFiles', [
      shape.PackageJsonFile({
        shortPath: 'package.json',
      }),
    ]);

    expect(ctx).toHaveProperty('packageJsonFilesByName', expect.any(Object));
  });
});
