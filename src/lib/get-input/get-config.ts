import {
  isArray,
  isArrayOfStrings,
  isBoolean,
  isNonEmptyString,
  isObject,
  isString,
} from 'expect-more';
import type {
  SemverGroup,
  SyncpackConfig,
  ValidRange,
  VersionGroup,
} from '../../constants';
import { DEFAULT_CONFIG, DEPENDENCY_TYPES } from '../../constants';
import { isValidSemverRange } from '../../lib/is-semver';
import type { Disk } from '../disk';

/**
 * Take all configuration from the command line and config file, combine it, and
 * set defaults for anything which hasn't been defined.
 *
 * @param  rcFile   Optional configuration file contents
 * @param  program  Optional command line options
 */
export const getConfig = (
  disk: Disk,
  program: Partial<SyncpackConfig & { configPath: string }>,
): SyncpackConfig => {
  type OptionName = keyof SyncpackConfig;
  type TypeChecker<T> = (value: unknown) => value is T;

  const rcFile = disk.readConfigFileSync(program.configPath);

  const hasTypeOverride =
    isBoolean(program.dev) ||
    isBoolean(program.workspace) ||
    isBoolean(program.overrides) ||
    isBoolean(program.peer) ||
    isBoolean(program.prod) ||
    isBoolean(program.resolutions);

  const dev = hasTypeOverride
    ? Boolean(program.dev)
    : getOption<boolean>('dev', isBoolean);
  const workspace = hasTypeOverride
    ? Boolean(program.workspace)
    : getOption<boolean>('workspace', isBoolean);
  const overrides = hasTypeOverride
    ? Boolean(program.overrides)
    : getOption<boolean>('overrides', isBoolean);
  const peer = hasTypeOverride
    ? Boolean(program.peer)
    : getOption<boolean>('peer', isBoolean);
  const prod = hasTypeOverride
    ? Boolean(program.prod)
    : getOption<boolean>('prod', isBoolean);
  const resolutions = hasTypeOverride
    ? Boolean(program.resolutions)
    : getOption<boolean>('resolutions', isBoolean);

  const dependencyTypes =
    dev || workspace || overrides || peer || prod || resolutions
      ? DEPENDENCY_TYPES.filter(
          (type) =>
            (type === 'devDependencies' && dev) ||
            (type === 'overrides' && overrides) ||
            (type === 'peerDependencies' && peer) ||
            (type === 'dependencies' && prod) ||
            (type === 'resolutions' && resolutions),
        )
      : DEPENDENCY_TYPES;

  const filter = getOption<string>('filter', isNonEmptyString);
  const indent = getOption<string>('indent', isNonEmptyString);
  const semverRange = getOption<ValidRange>('semverRange', isValidSemverRange);
  const sortAz = getOption<string[]>('sortAz', isArrayOfStrings);
  const sortFirst = getOption<string[]>('sortFirst', isArrayOfStrings);
  const source = getOption<string[]>('source', isArrayOfStrings);

  /**
   * Every instance must belong to a semver group, even if that semver group is
   * this one which represents a default, no special treatment group.
   */
  const defaultSemverGroup = {
    range: semverRange,
    dependencies: ['**'],
    packages: ['**'],
  };

  const semverGroups = getOption<SemverGroup[]>(
    'semverGroups',
    isArrayOfSemverGroups,
  ).concat(defaultSemverGroup);

  /**
   * Every instance must belong to a semver group, even if that semver group is
   * this one which represents a default, no special treatment group.
   */
  const defaultVersionGroup = {
    packages: ['**'],
    dependencies: ['**'],
  };

  const versionGroups = getOption<VersionGroup[]>(
    'versionGroups',
    isArrayOfVersionGroups,
  ).concat(defaultVersionGroup);

  return {
    dependencyTypes,
    dev,
    filter,
    indent,
    workspace,
    overrides,
    peer,
    prod,
    resolutions,
    semverGroups,
    semverRange,
    sortAz,
    sortFirst,
    source,
    versionGroups,
  };

  function getOption<T>(name: OptionName, isValid: TypeChecker<T>): T {
    const cliOption = program[name];
    if (isValid(cliOption)) return cliOption;
    const configOption = rcFile[name];
    if (isValid(configOption)) return configOption;
    return DEFAULT_CONFIG[name] as unknown as T;
  }

  function isArrayOfSemverGroups(value: unknown): value is SemverGroup[] {
    return (
      isArray(value) &&
      value.every(
        (value: unknown) =>
          isObject(value) &&
          isArrayOfStrings(value.packages) &&
          isArrayOfStrings(value.dependencies) &&
          isString(value.range),
      )
    );
  }

  function isArrayOfVersionGroups(value: unknown): value is VersionGroup[] {
    return (
      isArray(value) &&
      value.every(
        (value: unknown) =>
          isObject(value) &&
          isArrayOfStrings(value.packages) &&
          isArrayOfStrings(value.dependencies),
      )
    );
  }
};
