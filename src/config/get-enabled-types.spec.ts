import { Effect, pipe } from 'effect';
import { InvalidCustomTypeError } from './get-custom-types';
import { DeprecatedTypesError, getEnabledTypes, RenamedWorkspaceTypeError } from './get-enabled-types';

const prod = expect.objectContaining({
  path: 'dependencies',
});
const dev = expect.objectContaining({
  path: 'devDependencies',
});
const local = expect.objectContaining({
  path: 'version',
});
const overrides = expect.objectContaining({
  path: 'overrides',
});
const peerDependencies = expect.objectContaining({
  path: 'peerDependencies',
});
const pnpmOverrides = expect.objectContaining({
  path: 'pnpm.overrides',
});
const resolutions = expect.objectContaining({
  path: 'resolutions',
});
// custom
const engines = expect.objectContaining({
  path: 'engines',
});
const packageManager = expect.objectContaining({
  path: 'packageManager',
});
const specificEngine = expect.objectContaining({
  path: 'engines.node',
});
const nameAndVersion = expect.objectContaining({
  namePath: 'name',
  path: 'version',
});

it('defaults to all when nothing is provided', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {},
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev, local, overrides, peerDependencies, pnpmOverrides, prod, resolutions]);
});

it('uses every type except a negated type such as "!prod"', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: '!prod',
          },
          rcFile: {},
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev, local, overrides, peerDependencies, pnpmOverrides, resolutions]);
});

it('handles multiple negated types', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: '!prod,!dev',
          },
          rcFile: {},
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([local, overrides, peerDependencies, pnpmOverrides, resolutions]);
});

it('uses only provided type when defined', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: 'dev',
          },
          rcFile: {},
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev]);
});

it('handles multiple types', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: 'dev,peer',
          },
          rcFile: {},
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev, peerDependencies]);
});

it('gives precedence to cli options', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: 'dev',
          },
          rcFile: {
            dependencyTypes: ['peer'],
          },
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev]);
});

it('includes custom types when others are negated', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: '!dev',
          },
          rcFile: {
            customTypes: {
              engines: {
                path: 'engines',
                strategy: 'versionsByName',
              },
            },
          },
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([local, overrides, peerDependencies, pnpmOverrides, prod, resolutions, engines]);
});

it('includes custom types when named', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            dependencyTypes: ['dev', 'engines'],
            customTypes: {
              engines: {
                path: 'engines',
                strategy: 'versionsByName',
              },
            },
          },
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev, engines]);
});

it('includes every type when "**" is provided', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {
            types: '**',
          },
          rcFile: {
            customTypes: {
              engines: {
                path: 'engines',
                strategy: 'versionsByName',
              },
            },
          },
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev, local, overrides, peerDependencies, pnpmOverrides, prod, resolutions, engines]);
});

it('includes every kind of custom type when named', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            dependencyTypes: ['dev', 'engines', 'packageManager', 'specificEngine', 'nameAndVersion'],
            customTypes: {
              engines: {
                path: 'engines',
                strategy: 'versionsByName',
              },
              packageManager: {
                path: 'packageManager',
                strategy: 'name@version',
              },
              specificEngine: {
                path: 'engines.node',
                strategy: 'version',
              },
              nameAndVersion: {
                namePath: 'name',
                path: 'version',
                strategy: 'name~version',
              },
            },
          },
        }),
        Effect.merge,
      ),
    ),
  ).toEqual([dev, engines, packageManager, specificEngine, nameAndVersion]);
});

it('returns error when deprecated boolean configs are used', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            prod: true,
          } as any,
        }),
        Effect.merge,
      ),
    ),
  ).toEqual(
    new DeprecatedTypesError({
      types: ['prod'],
    }),
  );
});

it('returns error when deprecated "workspace" name is used', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            dependencyTypes: ['workspace'],
          },
        }),
        Effect.merge,
      ),
    ),
  ).toEqual(new RenamedWorkspaceTypeError({}));
});

it('returns error when custom type is not an object', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            customTypes: {
              bad: null,
            },
          } as any,
        }),
        Effect.merge,
      ),
    ),
  ).toEqual(
    new InvalidCustomTypeError({
      config: null,
      reason: 'Invalid customType',
    }),
  );
});

it('returns error when custom type has invalid path', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            customTypes: {
              bad: {},
            },
          } as any,
        }),
        Effect.merge,
      ),
    ),
  ).toEqual(
    new InvalidCustomTypeError({
      config: {},
      reason: 'Invalid customType.path',
    }),
  );
});

it('returns error when custom type has invalid strategy', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            customTypes: {
              bad: {
                path: 'some.prop',
              },
            },
          } as any,
        }),
        Effect.merge,
      ),
    ),
  ).toEqual(
    new InvalidCustomTypeError({
      config: {
        path: 'some.prop',
      },
      reason: 'Invalid customType.strategy',
    }),
  );
});

it('returns error when custom name~version strategy has invalid path', () => {
  expect(
    Effect.runSync(
      pipe(
        getEnabledTypes({
          cli: {},
          rcFile: {
            customTypes: {
              bad: {
                path: 'some.prop',
                strategy: 'name~version',
              },
            },
          } as any,
        }),
        Effect.merge,
      ),
    ),
  ).toEqual(
    new InvalidCustomTypeError({
      config: {
        path: 'some.prop',
        strategy: 'name~version',
      },
      reason: 'Invalid customType.namePath',
    }),
  );
});
