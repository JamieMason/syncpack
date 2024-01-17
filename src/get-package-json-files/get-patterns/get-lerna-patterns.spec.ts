import { Effect, Option as O, pipe } from 'effect';
import { expect, it } from 'vitest';
import { createScenario, type TestScenario } from '../../../test/lib/create-scenario.js';
import { getLernaPatterns } from './get-lerna-patterns.js';

async function runScenario(getScenario: () => TestScenario) {
  const scenario = getScenario();
  return await Effect.runPromise(pipe(getLernaPatterns(scenario.io), Effect.merge));
}

it('returns strings when found', async () => {
  expect(
    await runScenario(
      createScenario({
        'lerna.json': {
          packages: ['apps/**'],
        },
      }),
    ),
  ).toEqual(O.some(['apps/**']));
});

it('returns none when lerna.json cannot be read', async () => {
  expect(await runScenario(createScenario({}))).toEqual(O.none());
});

it('returns none when lerna.json is not valid JSON', async () => {
  expect(
    await runScenario(
      createScenario({
        'lerna.json': 'NOT-JSON',
      }),
    ),
  ).toEqual(O.none());
});

it('returns none when data is valid JSON but the wrong shape', async () => {
  expect(
    await runScenario(
      createScenario({
        'lerna.json': {
          wrong: 'shape',
        },
      }),
    ),
  ).toEqual(O.none());
});
