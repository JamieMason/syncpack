import * as Effect from '@effect/io/Effect';
import type { SemverRangeEffects, VersionEffects } from '../src/create-program/effects';
import type { ErrorHandlers } from '../src/error-handlers/create-error-handlers';

export interface MockEnv {
  readonly askForChoice: jest.Mock<any, any>;
  readonly askForInput: jest.Mock<any, any>;
  readonly exitProcess: jest.Mock<any, any>;
  readonly globSync: jest.Mock<any, any>;
  readonly readConfigFileSync: jest.Mock<any, any>;
  readonly readFileSync: jest.Mock<any, any>;
  readonly readYamlFileSync: jest.Mock<any, any>;
  readonly writeFileSync: jest.Mock<any, any>;
}

export function createMockEnv(): MockEnv {
  return {
    askForChoice: jest.fn(() => Promise.resolve()),
    askForInput: jest.fn(() => Promise.resolve()),
    exitProcess: jest.fn(() => []),
    globSync: jest.fn(() => []),
    readConfigFileSync: jest.fn(() => ({})),
    readFileSync: jest.fn(() => ''),
    readYamlFileSync: jest.fn(() => ({})),
    writeFileSync: jest.fn(),
  };
}

export function createMockErrorHandlers(): ErrorHandlers<jest.Mock<any, any>> {
  return {
    DeprecatedTypesError: jest.fn(),
    GlobError: jest.fn(),
    JsonParseError: jest.fn(),
    NoSourcesFoundError: jest.fn(),
    ReadConfigFileError: jest.fn(),
    ReadFileError: jest.fn(),
    SemverGroupConfigError: jest.fn(),
    VersionGroupConfigError: jest.fn(),
    WriteFileError: jest.fn(),
  };
}

export function createMockSemverRangeEffects(): SemverRangeEffects {
  return {
    FilteredOut: jest.fn(() => Effect.unit()),
    Ignored: jest.fn(() => Effect.unit()),
    SemverRangeMismatch: jest.fn(() => Effect.unit()),
    TearDown: jest.fn(() => Effect.unit()),
    UnsupportedVersion: jest.fn(() => Effect.unit()),
    Valid: jest.fn(() => Effect.unit()),
    WorkspaceSemverRangeMismatch: jest.fn(() => Effect.unit()),
  };
}

export function createMockVersionEffects(): VersionEffects {
  return {
    Banned: jest.fn(() => Effect.unit()),
    FilteredOut: jest.fn(() => Effect.unit()),
    HighestSemverMismatch: jest.fn(() => Effect.unit()),
    Ignored: jest.fn(() => Effect.unit()),
    LowestSemverMismatch: jest.fn(() => Effect.unit()),
    PinnedMismatch: jest.fn(() => Effect.unit()),
    SameRangeMismatch: jest.fn(() => Effect.unit()),
    SnappedToMismatch: jest.fn(() => Effect.unit()),
    TearDown: jest.fn(() => Effect.unit()),
    UnsupportedMismatch: jest.fn(() => Effect.unit()),
    Valid: jest.fn(() => Effect.unit()),
    WorkspaceMismatch: jest.fn(() => Effect.unit()),
  };
}
