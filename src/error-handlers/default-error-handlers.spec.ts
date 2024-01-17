import { Effect } from 'effect';
import { describe, expect, it, vi } from 'vitest';
import { createScenario } from '../../test/lib/create-scenario.js';
import { fixMismatches } from '../bin-fix-mismatches/fix-mismatches.js';
import { listMismatches } from '../bin-list-mismatches/list-mismatches.js';
import { list } from '../bin-list/list.js';

describe('Disk Errors', () => {
  describe('when a file cannot be read', () => {
    const getScenario = createScenario({
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('list', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        vi.spyOn(scenario.fs, 'readFileSync').mockImplementation(() => {
          throw new Error('wat');
        });
        await Effect.runPromiseExit(list(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        expect(scenario.errorHandlers.ReadFileError).toHaveBeenCalledWith({
          _tag: 'ReadFileError',
          error: 'Error: wat',
          filePath: expect.stringContaining('/package.json'),
        });
      });
    });

    describe('list-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        vi.spyOn(scenario.fs, 'readFileSync').mockImplementation(() => {
          throw new Error('wat');
        });
        await Effect.runPromiseExit(listMismatches(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        expect(scenario.errorHandlers.ReadFileError).toHaveBeenCalledWith({
          _tag: 'ReadFileError',
          error: 'Error: wat',
          filePath: expect.stringContaining('/package.json'),
        });
      });
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        vi.spyOn(scenario.fs, 'readFileSync').mockImplementation(() => {
          throw new Error('wat');
        });
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
        expect(scenario.errorHandlers.ReadFileError).toHaveBeenCalledWith({
          _tag: 'ReadFileError',
          error: 'Error: wat',
          filePath: expect.stringContaining('/package.json'),
        });
      });
    });
  });

  describe('when a file cannot be written', () => {
    const getScenario = createScenario({
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
      'packages/a/package.json': {
        name: 'a',
        version: '0.0.0',
        dependencies: {
          foo: '1.1.1',
        },
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        vi.spyOn(scenario.fs, 'writeFileSync').mockImplementation(() => {
          throw new Error('wat');
        });
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.WriteFileError).toHaveBeenCalledWith({
          _tag: 'WriteFileError',
          error: 'Error: wat',
          filePath: expect.stringContaining('/packages/a/package.json'),
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });
});

describe('Broken Projects', () => {
  describe('when a file is not valid json', () => {
    const getScenario = createScenario({
      'package.json': '}INVALID{',
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.JsonParseError).toHaveBeenCalledWith({
          _tag: 'JsonParseError',
          error: expect.any(SyntaxError),
          filePath: expect.stringContaining('/package.json'),
          json: '}INVALID{',
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when no package.json files are found', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        source: ['/does/not/exist'],
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.NoSourcesFoundError).toHaveBeenCalledWith({
          _tag: 'NoSourcesFoundError',
          CWD: '/fake/dir',
          patterns: ['/does/not/exist/package.json'],
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });
});

describe('Config Errors', () => {
  describe('when deprecated config for the local dependencyType is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        dependencyTypes: ['workspace'],
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.RenamedWorkspaceTypeError).toHaveBeenCalledWith({
          _tag: 'RenamedWorkspaceTypeError',
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when invalid config for semverGroups is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        semverGroups: ['wrong'],
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.SemverGroupConfigError).toHaveBeenCalledWith({
          _tag: 'SemverGroupConfigError',
          config: 'wrong',
          error: 'config is not an object',
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when confusing config for semverGroups is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        semverGroups: [
          {
            isIgnored: true,
            range: '^',
          },
        ],
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.SemverGroupConfigError).toHaveBeenCalledWith({
          _tag: 'SemverGroupConfigError',
          config: {
            isIgnored: true,
            range: '^',
          },
          error: "it's unclear what kind of semver group you want, as it contains both isIgnored and range",
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when invalid config for versionGroups is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: ['wrong'],
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.VersionGroupConfigError).toHaveBeenCalledWith({
          _tag: 'VersionGroupConfigError',
          config: 'wrong',
          error: 'config is not an object',
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when confusing config for versionGroups is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        versionGroups: [
          {
            isIgnored: true,
            isBanned: true,
          },
        ],
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.VersionGroupConfigError).toHaveBeenCalledWith({
          _tag: 'VersionGroupConfigError',
          config: {
            isIgnored: true,
            isBanned: true,
          },
          error: "it's unclear what kind of version group you want, as it contains both isBanned and isIgnored",
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when deprecated config for dependencyTypes is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        dev: true,
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.DeprecatedTypesError).toHaveBeenCalledWith({
          _tag: 'DeprecatedTypesError',
          types: ['dev'],
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when an invalid glob is used', () => {
    const getScenario = createScenario(
      {
        '.syncpackrc': {
          source: ['}i#&*^%nvalid{'],
        },
        'package.json': {
          name: 'foo',
          version: '0.1.0',
        },
      },
      {},
    );

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        vi.spyOn(scenario.io.globby, 'sync').mockImplementation(() => {
          throw new Error('wat');
        });
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.GlobError).toHaveBeenCalledWith({
          _tag: 'GlobError',
          error: 'Error: wat',
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });

  describe('when invalid config for customTypes is used', () => {
    const getScenario = createScenario({
      '.syncpackrc': {
        customTypes: {
          someType: 'wrong',
        },
      },
      'package.json': {
        name: 'foo',
        version: '0.1.0',
      },
    });

    describe('fix-mismatches', () => {
      it('exits 1', async () => {
        const scenario = getScenario();
        await Effect.runPromiseExit(fixMismatches(scenario));
        expect(scenario.errorHandlers.InvalidCustomTypeError).toHaveBeenCalledWith({
          _tag: 'InvalidCustomTypeError',
          config: 'wrong',
          reason: 'Invalid customType',
        });
        expect(scenario.io.process.exit).toHaveBeenCalledWith(1);
      });
    });
  });
});
