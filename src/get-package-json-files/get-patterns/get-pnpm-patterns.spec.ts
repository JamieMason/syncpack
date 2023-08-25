import { Effect, Option as O, pipe } from 'effect';
import { createScenario, type TestScenario } from '../../../test/lib/create-scenario';
import { getPnpmPatterns } from './get-pnpm-patterns';

function runScenario(getScenario: () => TestScenario) {
  const scenario = getScenario();
  return Effect.runSync(pipe(getPnpmPatterns(scenario.io), Effect.merge));
}

it('returns strings when found', () => {
  expect(
    runScenario(
      createScenario({
        'pnpm-workspace.yaml': {
          packages: ['apps/**'],
        },
      }),
    ),
  ).toEqual(O.some(['apps/**']));
});

it('returns none when pnpm-workspace.yaml cannot be read', () => {
  expect(runScenario(createScenario({}))).toEqual(O.none());
});

it('returns none when pnpm-workspace.yaml is valid YAML but the wrong shape', () => {
  expect(
    runScenario(
      createScenario({
        'pnpm-workspace.yaml': {
          wrong: 'shape',
        },
      }),
    ),
  ).toEqual(O.none());
});

it('returns none when pnpm-workspace.yaml is invalid', () => {
  const getScenario = createScenario({
    'pnpm-workspace.yaml': {
      see: 'mockFn',
    },
  });
  const scenario = getScenario();
  scenario.mockIo.readYamlFile.sync.mockImplementation(() => {
    throw new Error('wat?');
  });
  expect(Effect.runSync(pipe(getPnpmPatterns(scenario.io), Effect.merge))).toEqual(O.none());
});
