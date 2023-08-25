import { Effect } from 'effect';
import type { TestScenario } from '../../test/lib/create-scenario';
import { createScenario } from '../../test/lib/create-scenario';
import { NamedVersionStringStrategy } from './named-version-string';

function getRootPackage(filesByName: TestScenario['filesByName']) {
  return createScenario(filesByName)().getRootPackage();
}

it('gets and sets a name and version from a single string', () => {
  const strategy = new NamedVersionStringStrategy('local', 'packageManager');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
      packageManager: 'yarn@1.2.3',
    },
  });
  const initial = [['yarn', '1.2.3']];
  const updated = [['yarn', '2.0.0']];
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(Effect.runSyncExit(strategy.write(file, ['yarn', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('gets and sets a name and version from a single string nested location', () => {
  const strategy = new NamedVersionStringStrategy('custom', 'deeper.versionNumber');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
      deeper: {
        versionNumber: 'bar@1.2.3',
      },
    },
  });
  const initial = [['bar', '1.2.3']];
  const updated = [['bar', '2.0.0']];
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(Effect.runSyncExit(strategy.write(file, ['bar', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('returns empty array when path is not found', () => {
  const strategy = new NamedVersionStringStrategy('local', 'never.gonna');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
    },
  });
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed([]));
});
