import chalk from 'chalk';
import { Effect } from 'effect';
import { EOL } from 'os';
import type { InvalidCustomTypeError } from '../config/get-custom-types.js';
import type {
  DeprecatedTypesError,
  RenamedWorkspaceTypeError,
} from '../config/get-enabled-types.js';
import type { NoSourcesFoundError } from '../get-package-json-files/get-file-paths.js';
import type { GlobError } from '../io/glob-sync.js';
import type { ReadFileError } from '../io/read-file-sync.js';
import type { JsonParseError } from '../io/read-json-file-sync.js';
import type { WriteFileError } from '../io/write-file-sync.js';
import type { SemverGroup } from '../semver-group/index.js';
import type { VersionGroup } from '../version-group/index.js';

type R = Effect.Effect<never, never, void>;

export interface ErrorHandlers {
  // getInstances
  DeprecatedTypesError(err: DeprecatedTypesError): R;
  InvalidCustomTypeError(err: InvalidCustomTypeError): R;
  RenamedWorkspaceTypeError(err: RenamedWorkspaceTypeError): R;
  SemverGroupConfigError(err: SemverGroup.ConfigError): R;
  VersionGroupConfigError(err: VersionGroup.ConfigError): R;
  // getContext
  GlobError(err: GlobError): R;
  JsonParseError(err: JsonParseError): R;
  NoSourcesFoundError(err: NoSourcesFoundError): R;
  ReadFileError(err: ReadFileError): R;
  // writeFileIfChanged
  WriteFileError(err: WriteFileError): R;
}

export const defaultErrorHandlers: ErrorHandlers = {
  // getContext
  GlobError(err) {
    return Effect.logError(
      [
        chalk.red('An error was found when processing your source globs'),
        chalk.red('  Error:', err.error),
      ].join(EOL),
    );
  },
  JsonParseError(err) {
    return Effect.logError(
      [
        chalk.red('An error was found when parsing a JSON file'),
        chalk.red('  File:', err.filePath),
        chalk.red('  Error:', err.error),
      ].join(EOL),
    );
  },
  NoSourcesFoundError(err) {
    return Effect.logError(
      [
        chalk.red('No package.json files were found'),
        chalk.red('  CWD:', err.CWD),
        chalk.red('  Sources:', err.patterns),
      ].join(EOL),
    );
  },
  ReadFileError(err) {
    return Effect.logError(
      [
        chalk.red('An error was found when reading a file'),
        chalk.red('  File:', err.filePath),
        chalk.red('  Error:', err.error),
      ].join(EOL),
    );
  },
  // Others
  DeprecatedTypesError(err) {
    const url = 'https://github.com/JamieMason/syncpack/releases/tag/9.0.0';
    return Effect.logError(
      [
        chalk.red(`Your syncpack config file contains values deprecated in ${url}`),
        chalk.red('  Dependency Types:', err.types),
        chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/dependency-types'),
      ].join(EOL),
    );
  },
  InvalidCustomTypeError(err) {
    return Effect.logError(
      [
        chalk.red('Your syncpack config file contains an invalid custom type'),
        chalk.red('  Error:', err.reason),
        chalk.red('  Config:', err.config),
        chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/custom-types'),
      ].join(EOL),
    );
  },
  RenamedWorkspaceTypeError() {
    const url = 'https://github.com/JamieMason/syncpack/releases/tag/11.2.1';
    return Effect.logError(
      [
        chalk.red(`The "workspace" dependency type was renamed to "local" in ${url}`),
        chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/dependency-types'),
      ].join(EOL),
    );
  },
  SemverGroupConfigError(err) {
    return Effect.logError(
      [
        chalk.red('Your semver group config contains an error'),
        chalk.red('  Error:', err.error),
        chalk.red('  Config:', err.config),
        chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/semver-groups'),
      ].join(EOL),
    );
  },
  VersionGroupConfigError(err) {
    return Effect.logError(
      [
        chalk.red('Your version group config contains an error'),
        chalk.red('  Error:', err.error),
        chalk.red('  Config:', err.config),
        chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/version-groups'),
      ].join(EOL),
    );
  },
  WriteFileError(err) {
    return Effect.logError(
      [
        chalk.red('An error was found when writing to a file'),
        chalk.red('  File:', err.filePath),
        chalk.red('  Error:', err.error),
      ].join(EOL),
    );
  },
};
