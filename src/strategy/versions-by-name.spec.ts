import { Effect } from 'effect';
import type { TestScenario } from '../../test/lib/create-scenario';
import { createScenario } from '../../test/lib/create-scenario';
import { VersionsByNameStrategy } from './versions-by-name';

function getRootPackage(filesByName: TestScenario['filesByName']) {
  return createScenario(filesByName)().getRootPackage();
}

it('gets and sets names and versions in an object', () => {
  const strategy = new VersionsByNameStrategy('local', 'dependencies');
  const file = getRootPackage({
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
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(Effect.runSyncExit(strategy.write(file, ['bar', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('gets and sets a name and version from a single string nested location', () => {
  const strategy = new VersionsByNameStrategy('custom', 'deeper.deps');
  const file = getRootPackage({
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
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(Effect.runSyncExit(strategy.write(file, ['bar', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('returns empty array when path is not found', () => {
  const strategy = new VersionsByNameStrategy('local', 'never.gonna');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
    },
  });
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed([]));
});
