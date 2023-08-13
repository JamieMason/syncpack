import { getEnabledTypes } from './get-enabled-types';
import * as Effect from '@effect/io/Effect';

const prod = expect.objectContaining({ path: 'dependencies' });
const dev = expect.objectContaining({ path: 'devDependencies' });
const local = expect.objectContaining({ path: 'version' });
const overrides = expect.objectContaining({ path: 'overrides' });
const peerDependencies = expect.objectContaining({ path: 'peerDependencies' });
const pnpmOverrides = expect.objectContaining({ path: 'pnpm.overrides' });
const resolutions = expect.objectContaining({ path: 'resolutions' });
// custom
const engines = expect.objectContaining({ path: 'engines' });

it('defaults to all when nothing is provided', () => {
  expect(getEnabledTypes({ cli: {}, rcFile: {} })).toEqual(
    Effect.succeed([dev, local, overrides, peerDependencies, pnpmOverrides, prod, resolutions]),
  );
});

it('uses every type except a negated type such as "!prod"', () => {
  expect(getEnabledTypes({ cli: { types: '!prod' }, rcFile: {} })).toEqual(
    Effect.succeed([dev, local, overrides, peerDependencies, pnpmOverrides, resolutions]),
  );
});

it('handles multiple negated types', () => {
  expect(getEnabledTypes({ cli: { types: '!prod,!dev' }, rcFile: {} })).toEqual(
    Effect.succeed([local, overrides, peerDependencies, pnpmOverrides, resolutions]),
  );
});

it('uses only provided type when defined', () => {
  expect(getEnabledTypes({ cli: { types: 'dev' }, rcFile: {} })).toEqual(Effect.succeed([dev]));
});

it('handles multiple types', () => {
  expect(getEnabledTypes({ cli: { types: 'dev,peer' }, rcFile: {} })).toEqual(
    Effect.succeed([dev, peerDependencies]),
  );
});

it('gives precedence to cli options', () => {
  expect(getEnabledTypes({ cli: { types: 'dev' }, rcFile: { dependencyTypes: ['peer'] } })).toEqual(
    Effect.succeed([dev]),
  );
});

it('includes custom types when others are negated', () => {
  expect(
    getEnabledTypes({
      cli: { types: '!dev' },
      rcFile: {
        customTypes: {
          engines: {
            path: 'engines',
            strategy: 'versionsByName',
          },
        },
      },
    }),
  ).toEqual(
    Effect.succeed([local, overrides, peerDependencies, pnpmOverrides, prod, resolutions, engines]),
  );
});

it('includes custom types when named', () => {
  expect(
    getEnabledTypes({
      cli: { types: 'dev,engines' },
      rcFile: {
        customTypes: {
          engines: {
            path: 'engines',
            strategy: 'versionsByName',
          },
        },
      },
    }),
  ).toEqual(Effect.succeed([dev, engines]));
});

it('includes every type when "**" is provided', () => {
  expect(
    getEnabledTypes({
      cli: { types: '**' },
      rcFile: {
        customTypes: {
          engines: {
            path: 'engines',
            strategy: 'versionsByName',
          },
        },
      },
    }),
  ).toEqual(
    Effect.succeed([
      dev,
      local,
      overrides,
      peerDependencies,
      pnpmOverrides,
      prod,
      resolutions,
      engines,
    ]),
  );
});
