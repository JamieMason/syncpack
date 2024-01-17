import { Effect } from 'effect';
import { expect, it } from 'vitest';
import type { TestScenario } from '../../test/lib/create-scenario.js';
import { createScenario } from '../../test/lib/create-scenario.js';
import { VersionsByNameStrategy } from './versions-by-name.js';

function getRootPackage(filesByName: TestScenario['filesByName']) {
  return createScenario(filesByName)().getRootPackage();
}

it('gets and sets names and versions in an object', async () => {
  const strategy = new VersionsByNameStrategy('local', 'dependencies');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
      dependencies: {
        bar: '1.2.3',
        baz: '4.4.4',
      },
    },
  });
  const initial = [
    ['bar', '1.2.3'],
    ['baz', '4.4.4'],
  ];
  const updated = [
    ['bar', '2.0.0'],
    ['baz', '4.4.4'],
  ];
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(await Effect.runPromiseExit(strategy.write(file, ['bar', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('gets and sets a name and version from a single string nested location', async () => {
  const strategy = new VersionsByNameStrategy('custom', 'deeper.deps');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
      deeper: {
        deps: {
          bar: '1.2.3',
          baz: '4.4.4',
        },
      },
    },
  });
  const initial = [
    ['bar', '1.2.3'],
    ['baz', '4.4.4'],
  ];
  const updated = [
    ['bar', '2.0.0'],
    ['baz', '4.4.4'],
  ];
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(await Effect.runPromiseExit(strategy.write(file, ['bar', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('returns empty array when path is not found', async () => {
  const strategy = new VersionsByNameStrategy('local', 'never.gonna');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
    },
  });
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed([]));
});
