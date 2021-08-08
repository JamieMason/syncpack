import { cosmiconfigSync } from 'cosmiconfig';
import { isArray, isArrayOfStrings, isBoolean, isNonEmptyString, isObject } from 'expect-more';
import { isValidSemverRange } from '../commands/lib/is-semver';
import { DEFAULT_CONFIG, SyncpackConfig, ValidRange, VersionGroup } from '../constants';

export const getConfig = (program: Partial<SyncpackConfig>): SyncpackConfig => {
  type OptionName = keyof SyncpackConfig;
  type TypeChecker<T> = (value: unknown) => value is T;

  const rcSearch = cosmiconfigSync('syncpack').search();
  const rcConfig = rcSearch !== null ? rcSearch.config : {};
  const rcFile = isObject<Partial<SyncpackConfig>>(rcConfig) ? rcConfig : {};

  const getOption = <T>(name: OptionName, isValid: TypeChecker<T>): T => {
    const cliOption = program[name];
    if (isValid(cliOption)) {
      return cliOption;
    }
    const configOption = rcFile[name];
    if (isValid(configOption)) {
      return configOption;
    }
    return DEFAULT_CONFIG[name] as unknown as T;
  };

  const isVersionGroup = (value: unknown): value is VersionGroup =>
    isObject(value) && isArrayOfStrings(value.packages) && isArrayOfStrings(value.dependencies);

  const isArrayOfVersionGroups = (value: unknown): value is VersionGroup[] =>
    isArray(value) && value.every(isVersionGroup);

  const hasTypeOverride = program.prod || program.dev || program.peer;

  return {
    dev: hasTypeOverride ? Boolean(program.dev) : getOption<boolean>('dev', isBoolean),
    filter: getOption<string>('filter', (value: unknown): value is string => isNonEmptyString(value)),
    indent: getOption<string>('indent', (value: unknown): value is string => isNonEmptyString(value)),
    peer: hasTypeOverride ? Boolean(program.peer) : getOption<boolean>('peer', isBoolean),
    prod: hasTypeOverride ? Boolean(program.prod) : getOption<boolean>('prod', isBoolean),
    semverRange: getOption<ValidRange>('semverRange', isValidSemverRange),
    sortAz: getOption<string[]>('sortAz', isArrayOfStrings),
    sortFirst: getOption<string[]>('sortFirst', isArrayOfStrings),
    source: getOption<string[]>('source', isArrayOfStrings),
    versionGroups: getOption<VersionGroup[]>('versionGroups', isArrayOfVersionGroups),
  };
};
