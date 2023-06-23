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

export function createMockSemverRangeEffects(): SemverRangeEffects<void> {
  return {
    onFilteredOut: jest.fn(() => Effect.unit()),
    onIgnored: jest.fn(() => Effect.unit()),
    onSemverRangeMismatch: jest.fn(() => Effect.unit()),
    onComplete: jest.fn(() => Effect.unit()),
    onUnsupportedVersion: jest.fn(() => Effect.unit()),
    onValid: jest.fn(() => Effect.unit()),
    onWorkspaceSemverRangeMismatch: jest.fn(() => Effect.unit()),
  };
}

export function createMockVersionEffects(): VersionEffects<void> {
  return {
    onBanned: jest.fn(() => Effect.unit()),
    onFilteredOut: jest.fn(() => Effect.unit()),
    onHighestSemverMismatch: jest.fn(() => Effect.unit()),
    onIgnored: jest.fn(() => Effect.unit()),
    onLowestSemverMismatch: jest.fn(() => Effect.unit()),
    onPinnedMismatch: jest.fn(() => Effect.unit()),
    onSameRangeMismatch: jest.fn(() => Effect.unit()),
    onSnappedToMismatch: jest.fn(() => Effect.unit()),
    onComplete: jest.fn(() => Effect.unit()),
    onUnsupportedMismatch: jest.fn(() => Effect.unit()),
    onValid: jest.fn(() => Effect.unit()),
    onWorkspaceMismatch: jest.fn(() => Effect.unit()),
  };
}
