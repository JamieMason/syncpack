import chalk from 'chalk';
import { ICON } from '../constants';
import type { ErrorHandlers } from './create-error-handlers';

export const defaultErrorHandlers: ErrorHandlers<void> = {
  DeprecatedTypesError(err) {
    const url = 'https://github.com/JamieMason/syncpack/releases/tag/9.0.0';
    console.log(
      chalk.red(ICON.panic, `Your syncpack config file contains values deprecated in ${url}`),
    );
    console.log(chalk.red('  Dependency Types:', err.types));
    console.log(chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/dependency-types'));
  },
  GlobError(err) {
    console.log(chalk.red(ICON.panic, 'An error was found when processing your source globs'));
    console.log(chalk.red('  Error:', err.error));
  },
  JsonParseError(err) {
    console.log(chalk.red(ICON.panic, 'An error was found when parsing a JSON file'));
    console.log(chalk.red('  File:', err.filePath));
    console.log(chalk.red('  Error:', err.error));
  },
  NoSourcesFoundError(err) {
    console.log(chalk.red(ICON.panic, 'No package.json files were found'));
    console.log(chalk.red('  CWD:'), err.CWD);
    console.log(chalk.red('  Sources:', err.patterns));
  },
  ReadConfigFileError(err) {
    console.log(chalk.red(ICON.panic, 'Your syncpack config file contains an error'));
    console.log(chalk.red('  File:', err.filePath || 'not known (discovered by cosmiconfig)'));
    console.log(chalk.red('  Error:', err.error));
  },
  ReadFileError(err) {
    console.log(chalk.red(ICON.panic, 'An error was found when reading a file'));
    console.log(chalk.red('  File:', err.filePath));
    console.log(chalk.red('  Error:', err.error));
  },
  SemverGroupConfigError(err) {
    console.log(chalk.red(ICON.panic, 'Your semver group config contains an error'));
    console.log(chalk.red('  Error:', err.error));
    console.log(chalk.red('  Config:'), err.config);
    console.log(chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/semver-groups'));
  },
  VersionGroupConfigError(err) {
    console.log(chalk.red(ICON.panic, 'Your version group config contains an error'));
    console.log(chalk.red('  Error:', err.error));
    console.log(chalk.red('  Config:'), err.config);
    console.log(chalk.red('  Docs: https://jamiemason.github.io/syncpack/config/version-groups'));
  },
  WriteFileError(err) {
    console.log(chalk.red(ICON.panic, 'An error was found when writing to a file'));
    console.log(chalk.red('  File:', err.filePath));
    console.log(chalk.red('  Error:', err.error));
  },
};
