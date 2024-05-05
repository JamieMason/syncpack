import { Effect, Option as O, pipe } from 'effect';
import { expect, it } from 'vitest';
import { createScenario, type TestScenario } from '../../../test/lib/create-scenario.js';
import { getPnpmPatterns } from './get-pnpm-patterns.js';

async function runScenario(getScenario: () => TestScenario) {
  const scenario = getScenario();
  return await Effect.runPromise(pipe(getPnpmPatterns(scenario.io), Effect.merge));
}

it('returns strings when found', async () => {
  expect(
    await runScenario(
      createScenario({
        'pnpm-workspace.yaml': {
          packages: ['apps/**'],
        },
      }),
    ),
  ).toEqual(O.some(['apps/**']));
});

it('returns none when pnpm-workspace.yaml cannot be read', async () => {
  expect(await runScenario(createScenario({}))).toEqual(O.none());
});

it('returns none when pnpm-workspace.yaml is valid YAML but the wrong shape', async () => {
  expect(
    await runScenario(
      createScenario({
        'pnpm-workspace.yaml': {
          wrong: 'shape',
        },
      }),
    ),
  ).toEqual(O.none());
});

it('returns none when pnpm-workspace.yaml is invalid', async () => {
  const getScenario = createScenario({
    'pnpm-workspace.yaml': {
      see: 'mockFn',
    },
  });
  const scenario = getScenario();
  scenario.mockIo.readYamlFile.sync.mockImplementation(() => {
    throw new Error('wat?');
  });
  expect(await Effect.runPromise(pipe(getPnpmPatterns(scenario.io), Effect.merge))).toEqual(O.none());
});
