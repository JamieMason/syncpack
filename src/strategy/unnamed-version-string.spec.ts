import { Effect } from 'effect';
import type { TestScenario } from '../../test/lib/create-scenario';
import { createScenario } from '../../test/lib/create-scenario';
import { UnnamedVersionStringStrategy } from './unnamed-version-string';

function getRootPackage(filesByName: TestScenario['filesByName']) {
  return createScenario(filesByName)().getRootPackage();
}

it('gets and sets an anonymous version from a single string', () => {
  const strategy = new UnnamedVersionStringStrategy('local', 'someVersion');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
      someVersion: '1.2.3',
    },
  });
  const initial = [['someVersion', '1.2.3']];
  const updated = [['someVersion', '2.0.0']];
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(Effect.runSyncExit(strategy.write(file, ['someVersion', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('gets and sets an anonymous version from a single string in a nested location', () => {
  const strategy = new UnnamedVersionStringStrategy('custom', 'engines.node');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
      engines: {
        node: '1.2.3',
      },
    },
  });
  const initial = [['node', '1.2.3']];
  const updated = [['node', '2.0.0']];
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(Effect.runSyncExit(strategy.write(file, ['node', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('returns empty array when path is not found', () => {
  const strategy = new UnnamedVersionStringStrategy('local', 'never.gonna');
  const file = getRootPackage({
    'package.json': {
      name: 'foo',
    },
  });
  expect(Effect.runSyncExit(strategy.read(file))).toEqual(Effect.succeed([]));
});
