import { cosmiconfigSync } from 'cosmiconfig';
import { isArray, isBoolean, isNonEmptyString, isObject, isRegExp } from 'expect-more';
import { isValidSemverRange } from '../commands/lib/is-semver';
import { DEFAULT_CONFIG, SyncpackConfig } from '../constants';

export interface Options {
  dev: boolean;
  filter: string;
  indent: string;
  peer: boolean;
  prod: boolean;
  semverRange: string;
  source: string[];
}

export const getConfig = (program: Partial<Options>): SyncpackConfig => {
  type OptionName = keyof Options;
  type TypeChecker = (value: any) => boolean;

  const rcSearch = cosmiconfigSync('syncpack').search();
  const rcFile = isObject(rcSearch) && isObject(rcSearch.config) ? rcSearch.config : {};
  const rcOptions = isObject(rcFile.options) ? rcFile.options : {};

  const getOption = (name: OptionName, isValid: TypeChecker) => {
    if (isValid(program[name])) {
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
    source: getOption('source', isArray),
  };

  if (program.prod || program.dev || program.peer) {
    config.prod = Boolean(program.prod);
    config.dev = Boolean(program.dev);
    config.peer = Boolean(program.peer);
  }

  return config;
};
