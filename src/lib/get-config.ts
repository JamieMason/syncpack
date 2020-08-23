import { cosmiconfigSync } from 'cosmiconfig';
import { isArray, isArrayOfStrings, isBoolean, isNonEmptyString, isObject, isRegExp } from 'expect-more';
import { isValidSemverRange } from '../commands/lib/is-semver';
import { DEFAULT_CONFIG, SyncpackConfig } from '../constants';

export interface CliOptions {
  dev: boolean;
  filter: string;
  indent: string;
  peer: boolean;
  prod: boolean;
  semverRange: string;
  source: string[];
}

export const getConfig = (program: Partial<CliOptions>): SyncpackConfig => {
  type OptionName = keyof CliOptions | 'sortAz' | 'sortFirst';
  type TypeChecker = (value: any) => boolean;

  const rcSearch = cosmiconfigSync('syncpack').search();
  const rcFile = isObject(rcSearch) && isObject(rcSearch.config) ? rcSearch.config : {};
  const rcOptions = isObject(rcFile.options) ? rcFile.options : {};

  const isCliOption = (key: string): key is keyof CliOptions => key in program;

  const getOption = (name: OptionName, isValid: TypeChecker) => {
    if (isCliOption(name) && isValid(program[name])) {
      return program[name];
    }
    if (isValid(rcOptions[name])) {
      return rcOptions[name];
    }
    return DEFAULT_CONFIG[name];
  };

  const filter = getOption('filter', isNonEmptyString);

  const config = {
    dev: getOption('dev', isBoolean),
    filter: isRegExp(filter) ? filter : new RegExp(filter),
    indent: getOption('indent', isNonEmptyString),
    peer: getOption('peer', isBoolean),
    prod: getOption('prod', isBoolean),
    semverRange: getOption('semverRange', isValidSemverRange),
    sortAz: getOption('sortAz', isArrayOfStrings),
    sortFirst: getOption('sortFirst', isArrayOfStrings),
    source: getOption('source', isArray),
  };

  if (program.prod || program.dev || program.peer) {
    config.prod = Boolean(program.prod);
    config.dev = Boolean(program.dev);
    config.peer = Boolean(program.peer);
  }

  return config;
};
