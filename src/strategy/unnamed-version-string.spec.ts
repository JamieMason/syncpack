import { Effect } from 'effect';
import { expect, it } from 'vitest';
import type { TestScenario } from '../../test/lib/create-scenario.js';
import { createScenario } from '../../test/lib/create-scenario.js';
import { UnnamedVersionStringStrategy } from './unnamed-version-string.js';

function getRootPackage(filesByName: TestScenario['filesByName']) {
  return createScenario(filesByName)().getRootPackage();
}

it('gets and sets an anonymous version from a single string', async () => {
  const strategy = new UnnamedVersionStringStrategy('local', 'someVersion');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
      someVersion: '1.2.3',
    },
  });
  const initial = [['someVersion', '1.2.3']];
  const updated = [['someVersion', '2.0.0']];
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(await Effect.runPromiseExit(strategy.write(file, ['someVersion', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('gets and sets an anonymous version from a single string in a nested location', async () => {
  const strategy = new UnnamedVersionStringStrategy('custom', 'engines.node');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
      engines: {
        node: '1.2.3',
      },
    },
  });
  const initial = [['node', '1.2.3']];
  const updated = [['node', '2.0.0']];
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(await Effect.runPromiseExit(strategy.write(file, ['node', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('returns empty array when path is not found', async () => {
  const strategy = new UnnamedVersionStringStrategy('local', 'never.gonna');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
    },
  });
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed([]));
});
