import { Effect, Option as O, pipe } from 'effect';
import { createScenario, type TestScenario } from '../../../test/lib/create-scenario';
import { getLernaPatterns } from './get-lerna-patterns';

function runScenario(getScenario: () => TestScenario) {
  const scenario = getScenario();
  return Effect.runSync(pipe(getLernaPatterns(scenario.io), Effect.merge));
}

it('returns strings when found', () => {
  expect(
    runScenario(
      createScenario({
        'lerna.json': {
          packages: ['apps/**'],
        },
      }),
    ),
  ).toEqual(O.some(['apps/**']));
});

it('returns none when lerna.json cannot be read', () => {
  expect(runScenario(createScenario({}))).toEqual(O.none());
});

it('returns none when lerna.json is not valid JSON', () => {
  expect(
    runScenario(
      createScenario({
        'lerna.json': 'NOT-JSON',
      }),
    ),
  ).toEqual(O.none());
});

it('returns none when data is valid JSON but the wrong shape', () => {
  expect(
    runScenario(
      createScenario({
        'lerna.json': {
          wrong: 'shape',
        },
      }),
    ),
  ).toEqual(O.none());
});
