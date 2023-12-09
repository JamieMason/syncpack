import { Effect, Option as O, pipe } from 'effect';
import { describe, expect, it } from 'vitest';
import type { TestScenario } from '../../../test/lib/create-scenario';
import { createScenario } from '../../../test/lib/create-scenario';
import { getYarnPatterns } from './get-yarn-patterns';

function runScenario(getScenario: () => TestScenario) {
  const scenario = getScenario();
  return Effect.runSync(pipe(getYarnPatterns(scenario.io), Effect.merge));
}

describe('when Yarn config is at .workspaces[]', () => {
  it('returns strings when found', () => {
    expect(
      runScenario(
        createScenario({
          'package.json': {
            workspaces: ['apps/**'],
          },
        }),
      ),
    ).toEqual(O.some(['apps/**']));
  });

  it('returns none when data is valid JSON but the wrong shape', () => {
    expect(
      runScenario(
        createScenario({
          'package.json': {
            wrong: 'shape',
          },
        }),
      ),
    ).toEqual(O.none());
  });
});

describe('when Yarn config is at .workspaces.packages[]', () => {
  it('returns an strings when found', () => {
    expect(
      runScenario(
        createScenario({
          'package.json': {
            workspaces: {
              packages: ['apps/**'],
            },
          },
        }),
      ),
    ).toEqual(O.some(['apps/**']));
  });

  it('returns none when data is valid JSON but the wrong shape', () => {
    expect(
      runScenario(
        createScenario({
          'package.json': {
            workspaces: {
              packages: 'wrong',
            },
          },
        }),
      ),
    ).toEqual(O.none());
  });
});

it('returns none when package.json cannot be read', () => {
  expect(runScenario(createScenario({}))).toEqual(O.none());
});

it('returns none when file is not valid JSON', () => {
  expect(
    runScenario(
      createScenario({
        'package.json': 'NOT-JSON',
      }),
    ),
  ).toEqual(O.none());
});
