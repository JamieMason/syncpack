import { Effect } from 'effect';
import { expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario';
import { format } from './format';

it('uses github shorthand format for "repository"', () => {
  const scenario = createScenario({
    'package.json': {
      name: 'a',
      repository: {
        url: 'git://github.com/User/repo',
        type: 'git',
      },
    },
  })();
  Effect.runSyncExit(format(scenario));

  expect(scenario.readPackages()).toHaveProperty('a', {
    name: 'a',
    repository: 'User/repo',
  });
  expect(scenario.io.process.exit).not.toHaveBeenCalled();
});

it('retains long form format for "repository" when directory property used', () => {
  const scenario = createScenario({
    'package.json': {
      name: 'a',
      repository: {
        url: 'git://gitlab.com/User/repo',
        type: 'git',
        directory: 'packages/foo',
      },
    },
  })();
  Effect.runSyncExit(format(scenario));
  expect(scenario.readPackages()).toHaveProperty('a', {
    name: 'a',
    repository: {
      url: 'git://gitlab.com/User/repo',
      type: 'git',
      directory: 'packages/foo',
    },
  });
  expect(scenario.io.process.exit).not.toHaveBeenCalled();
});

it('uses shorthand format for "bugs" and "repository"', () => {
  const scenario = createScenario({
    'package.json': {
      name: 'a',
      bugs: {
        url: 'https://github.com/User/repo/issues',
      },
      repository: {
        url: 'git://gitlab.com/User/repo',
        type: 'git',
      },
    },
  })();
  Effect.runSyncExit(format(scenario));
  expect(scenario.readPackages()).toHaveProperty('a', {
    name: 'a',
    bugs: 'https://github.com/User/repo/issues',
    repository: 'git://gitlab.com/User/repo',
  });
  expect(scenario.io.process.exit).not.toHaveBeenCalled();
});

it('sorts object properties alphabetically by key', () => {
  const scenario = createScenario({
    '.syncpackrc': {
      sortAz: ['scripts'],
    },
    'package.json': {
      name: 'a',
      scripts: {
        B: '',
        A: '',
      },
    },
  })();
  Effect.runSyncExit(format(scenario));
  expect(scenario.readPackages()).toHaveProperty('a.scripts', {
    A: '',
    B: '',
  });
  expect(scenario.io.process.exit).not.toHaveBeenCalled();
});

it('sorts array members alphabetically by value', () => {
  const scenario = createScenario({
    '.syncpackrc': {
      sortAz: ['keywords'],
    },
    'package.json': {
      name: 'a',
      keywords: ['B', 'A'],
    },
  })();
  Effect.runSyncExit(format(scenario));
  expect(scenario.readPackages()).toHaveProperty('a', {
    name: 'a',
    keywords: ['A', 'B'],
  });
  expect(scenario.io.process.exit).not.toHaveBeenCalled();
});

it('sorts named properties first, then the rest alphabetically', () => {
  const scenario = createScenario({
    '.syncpackrc': {
      sortFirst: ['name', 'F', 'E', 'D'],
    },
    'package.json': {
      name: 'a',
      A: '',
      F: '',
      B: '',
      D: '',
      E: '',
    },
  })();
  Effect.runSyncExit(format(scenario));
  expect(scenario.readPackages()).toHaveProperty('a', {
    name: 'a',
    F: '',
    E: '',
    D: '',
    A: '',
    B: '',
  });
  expect(scenario.io.process.exit).not.toHaveBeenCalled();
});
