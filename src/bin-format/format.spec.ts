import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { DEFAULT_CONFIG } from '../constants.js';
import { format } from './format.js';

describe('formatBugs', () => {
  it('uses github shorthand format', async () => {
    const scenario = createScenario({
      'package.json': {
        name: 'a',
        bugs: {
          url: 'https://github.com/User/repo/issues',
        },
      },
    })();

    await Effect.runPromiseExit(format(scenario));
    expect(scenario.readPackages()).toHaveProperty('a.bugs', 'https://github.com/User/repo/issues');
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});

describe('formatRepository', () => {
  it('uses gitlab shorthand format', async () => {
    const scenario = createScenario({
      'package.json': {
        name: 'a',
        repository: {
          url: 'git://gitlab.com/User/repo',
          type: 'git',
        },
      },
    })();

    await Effect.runPromiseExit(format(scenario));
    expect(scenario.readPackages()).toHaveProperty('a.repository', 'git://gitlab.com/User/repo');
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });

  it('uses github shorthand format', async () => {
    const scenario = createScenario({
      'package.json': {
        name: 'a',
        repository: {
          url: 'git://github.com/User/repo',
          type: 'git',
        },
      },
    })();

    await Effect.runPromiseExit(format(scenario));
    expect(scenario.readPackages()).toHaveProperty('a.repository', 'User/repo');
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });

  it('retains long format when directory property used', async () => {
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

    await Effect.runPromiseExit(format(scenario));

    expect(scenario.readPackages()).toHaveProperty('a.repository', {
      url: 'git://gitlab.com/User/repo',
      type: 'git',
      directory: 'packages/foo',
    });
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});

describe('sortExports', () => {
  it('sorts conditional exports', async () => {
    const scenario = createScenario({
      '.syncpackrc': {
        sortExports: DEFAULT_CONFIG.sortExports,
      },
      'package.json': {
        name: 'a',
        exports: {
          require: './index-require.cjs',
          import: './index-module.js',
        },
      },
    })();

    await Effect.runPromiseExit(format(scenario));

    const packages: any = scenario.readPackages();
    expect(Object.keys(packages.a.exports)).toEqual(['import', 'require']);
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });

  it('sorts conditional exports subpaths', async () => {
    const scenario = createScenario({
      '.syncpackrc': {
        sortExports: DEFAULT_CONFIG.sortExports,
      },
      'package.json': {
        name: 'a',
        exports: {
          '.': './index.js',
          './feature.js': {
            default: './feature.js',
            node: './feature-node.js',
          },
        },
      },
    })();

    await Effect.runPromiseExit(format(scenario));

    const packages: any = scenario.readPackages();
    expect(Object.keys(packages.a.exports['./feature.js'])).toEqual(['node', 'default']);
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});

describe('sortAz', () => {
  it('sorts object properties alphabetically by key', async () => {
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

    await Effect.runPromiseExit(format(scenario));

    const packages: any = scenario.readPackages();
    expect(Object.keys(packages.a.scripts)).toEqual(['A', 'B']);
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });

  it('sorts array members alphabetically by value', async () => {
    const scenario = createScenario({
      '.syncpackrc': {
        sortAz: ['keywords'],
      },
      'package.json': {
        name: 'a',
        keywords: ['B', 'A'],
      },
    })();

    await Effect.runPromiseExit(format(scenario));

    expect(scenario.readPackages()).toHaveProperty('a', {
      name: 'a',
      keywords: ['A', 'B'],
    });
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});

describe('sortFirst', () => {
  it('sorts named root properties first, leaving the rest alone', async () => {
    const scenario = createScenario({
      '.syncpackrc': {
        sortFirst: ['name', 'F', 'E', 'D'],
        sortPackages: false,
      },
      'package.json': {
        name: 'a',
        D: '',
        B: '',
        F: '',
        A: '',
        E: '',
      },
    })();

    await Effect.runPromiseExit(format(scenario));

    const packages: any = scenario.readPackages();
    expect(Object.keys(packages.a)).toEqual(['name', 'F', 'E', 'D', 'B', 'A']);
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});

describe('sortPackages', () => {
  it('sorts root properties alphabetically', async () => {
    const scenario = createScenario({
      '.syncpackrc': {
        sortFirst: [],
        sortPackages: true,
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

    await Effect.runPromiseExit(format(scenario));

    const packages: any = scenario.readPackages();
    expect(Object.keys(packages.a)).toEqual(['A', 'B', 'D', 'E', 'F', 'name']);
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});

describe('sortFirst and sortPackages together', () => {
  it('sorts named properties first, then the rest alphabetically', async () => {
    const scenario = createScenario({
      '.syncpackrc': {
        sortFirst: ['name', 'F', 'E', 'D'],
        sortPackages: true,
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

    await Effect.runPromiseExit(format(scenario));

    const packages: any = scenario.readPackages();
    expect(Object.keys(packages.a)).toEqual(['name', 'F', 'E', 'D', 'A', 'B']);
    expect(scenario.io.process.exit).not.toHaveBeenCalled();
  });
});
