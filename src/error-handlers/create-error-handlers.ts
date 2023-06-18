import * as Effect from '@effect/io/Effect';
import type { DeprecatedTypesError } from '../config/get-enabled-types';
import type { GlobError, ReadConfigFileError, ReadFileError, WriteFileError } from '../env/tags';
import type { NoSourcesFoundError } from '../get-package-json-files/get-file-paths';
import type { JsonParseError } from '../get-package-json-files/get-patterns/read-json-safe';
import type { SemverGroupConfigError } from '../get-semver-groups';
import type { VersionGroupConfigError } from '../get-version-groups';

export interface ErrorHandlers<R = Effect.Effect<never, never, void>> {
  DeprecatedTypesError(err: DeprecatedTypesError): R;
  GlobError(err: GlobError): R;
  JsonParseError(err: JsonParseError): R;
  NoSourcesFoundError(err: NoSourcesFoundError): R;
  ReadConfigFileError(err: ReadConfigFileError): R;
  ReadFileError(err: ReadFileError): R;
  SemverGroupConfigError(err: SemverGroupConfigError): R;
  VersionGroupConfigError(err: VersionGroupConfigError): R;
  WriteFileError(err: WriteFileError): R;
}

export const createErrorHandlers = (errorHandlers: ErrorHandlers<void>): ErrorHandlers => ({
  DeprecatedTypesError(err) {
    return Effect.sync(() => errorHandlers.DeprecatedTypesError(err));
  },
  GlobError(err) {
    return Effect.sync(() => errorHandlers.GlobError(err));
  },
  JsonParseError(err) {
    return Effect.sync(() => errorHandlers.JsonParseError(err));
  },
  NoSourcesFoundError(err) {
    return Effect.sync(() => errorHandlers.NoSourcesFoundError(err));
  },
  ReadConfigFileError(err) {
    return Effect.sync(() => errorHandlers.ReadConfigFileError(err));
  },
  ReadFileError(err) {
    return Effect.sync(() => errorHandlers.ReadFileError(err));
  },
  SemverGroupConfigError(err) {
    return Effect.sync(() => errorHandlers.SemverGroupConfigError(err));
  },
  VersionGroupConfigError(err) {
    return Effect.sync(() => errorHandlers.VersionGroupConfigError(err));
  },
  WriteFileError(err) {
    return Effect.sync(() => errorHandlers.WriteFileError(err));
  },
});
