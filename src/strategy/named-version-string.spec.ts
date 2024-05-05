import { Effect } from 'effect';
import { expect, it } from 'vitest';
import type { TestScenario } from '../../test/lib/create-scenario.js';
import { createScenario } from '../../test/lib/create-scenario.js';
import { NamedVersionStringStrategy } from './named-version-string.js';

function getRootPackage(filesByName: TestScenario['filesByName']) {
  return createScenario(filesByName)().getRootPackage();
}

it('gets and sets a name and version from a single string', async () => {
  const strategy = new NamedVersionStringStrategy('local', 'packageManager');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
      packageManager: 'yarn@1.2.3',
    },
  });
  const initial = [['yarn', '1.2.3']];
  const updated = [['yarn', '2.0.0']];
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(await Effect.runPromiseExit(strategy.write(file, ['yarn', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('gets and sets a name and version from a single string nested location', async () => {
  const strategy = new NamedVersionStringStrategy('custom', 'deeper.versionNumber');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
      deeper: {
        versionNumber: 'bar@1.2.3',
      },
    },
  });
  const initial = [['bar', '1.2.3']];
  const updated = [['bar', '2.0.0']];
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(initial));
  expect(await Effect.runPromiseExit(strategy.write(file, ['bar', '2.0.0']))).toEqual(Effect.succeed(file));
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed(updated));
});

it('returns empty array when path is not found', async () => {
  const strategy = new NamedVersionStringStrategy('local', 'never.gonna');
  const file = await getRootPackage({
    'package.json': {
      name: 'foo',
    },
  });
  expect(await Effect.runPromiseExit(strategy.read(file))).toEqual(Effect.succeed([]));
});
