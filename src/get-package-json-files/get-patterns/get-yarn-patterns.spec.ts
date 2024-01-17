import { Effect, Option as O, pipe } from 'effect';
import { describe, expect, it } from 'vitest';
import type { TestScenario } from '../../../test/lib/create-scenario.js';
import { createScenario } from '../../../test/lib/create-scenario.js';
import { getYarnPatterns } from './get-yarn-patterns.js';

async function runScenario(getScenario: () => TestScenario) {
  const scenario = getScenario();
  return await Effect.runPromise(pipe(getYarnPatterns(scenario.io), Effect.merge));
}

describe('when Yarn config is at .workspaces[]', () => {
  it('returns strings when found', async () => {
    expect(
      await runScenario(
        createScenario({
          'package.json': {
            workspaces: ['apps/**'],
          },
        }),
      ),
    ).toEqual(O.some(['apps/**']));
  });

  it('returns none when data is valid JSON but the wrong shape', async () => {
    expect(
      await runScenario(
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
  it('returns an strings when found', async () => {
    expect(
      await runScenario(
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

  it('returns none when data is valid JSON but the wrong shape', async () => {
    expect(
      await runScenario(
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

it('returns none when package.json cannot be read', async () => {
  expect(await runScenario(createScenario({}))).toEqual(O.none());
});

it('returns none when file is not valid JSON', async () => {
  expect(
    await runScenario(
      createScenario({
        'package.json': 'NOT-JSON',
      }),
    ),
  ).toEqual(O.none());
});
