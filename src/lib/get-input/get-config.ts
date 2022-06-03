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
import { verbose } from '../log';

/**
 * Take all configuration from the command line and config file, combine it, and
 * set defaults for anything which hasn't been defined.
 *
 * @param  rcFile   Optional configuration file contents
 * @param  program  Optional command line options
 */
export const getConfig = (
  disk: Disk,
  program: Partial<SyncpackConfig & { configPath: string | undefined }>,
): SyncpackConfig => {
  type OptionName = keyof SyncpackConfig;
  type TypeChecker<T> = (value: unknown) => value is T;

  verbose('cli arguments:', program);

  const rcFile = disk.readConfigFileSync(program.configPath);

  const hasTypeOverride =
    isBoolean(program.dev) ||
    isBoolean(program.overrides) ||
    isBoolean(program.peer) ||
    isBoolean(program.pnpmOverrides) ||
    isBoolean(program.prod) ||
    isBoolean(program.resolutions) ||
    isBoolean(program.workspace);

  const dev = hasTypeOverride
    ? Boolean(program.dev)
    : getOption<boolean>('dev', isBoolean);
  const overrides = hasTypeOverride
    ? Boolean(program.overrides)
    : getOption<boolean>('overrides', isBoolean);
  const peer = hasTypeOverride
    ? Boolean(program.peer)
    : getOption<boolean>('peer', isBoolean);
  const pnpmOverrides = hasTypeOverride
    ? Boolean(program.pnpmOverrides)
    : getOption<boolean>('pnpmOverrides', isBoolean);
  const prod = hasTypeOverride
    ? Boolean(program.prod)
    : getOption<boolean>('prod', isBoolean);
  const resolutions = hasTypeOverride
    ? Boolean(program.resolutions)
    : getOption<boolean>('resolutions', isBoolean);
  const workspace = hasTypeOverride
    ? Boolean(program.workspace)
    : getOption<boolean>('workspace', isBoolean);

  const dependencyTypes =
    dev ||
    overrides ||
    peer ||
    pnpmOverrides ||
    prod ||
    resolutions ||
    workspace
      ? DEPENDENCY_TYPES.filter(
          (type) =>
            (type === 'devDependencies' && dev) ||
            (type === 'overrides' && overrides) ||
            (type === 'peerDependencies' && peer) ||
            (type === 'pnpmOverrides' && pnpmOverrides) ||
            (type === 'dependencies' && prod) ||
            (type === 'resolutions' && resolutions) ||
            (type === 'workspace' && workspace),
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

  const finalConfig: SyncpackConfig = {
    dependencyTypes,
    dev,
    filter,
    indent,
    workspace,
    overrides,
    peer,
    pnpmOverrides,
    prod,
    resolutions,
    semverGroups,
    semverRange,
    sortAz,
    sortFirst,
    source,
    versionGroups,
  };

  verbose('final config:', finalConfig);

  return finalConfig;

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
