import {
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LOOSE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH,
  ValidRange,
} from '../../constants';

export const isValidSemverRange = (value: string): value is ValidRange =>
  value === RANGE_EXACT ||
  value === RANGE_GT ||
  value === RANGE_GTE ||
  value === RANGE_LOOSE ||
  value === RANGE_LT ||
  value === RANGE_LTE ||
  value === RANGE_MINOR ||
  value === RANGE_PATCH;

export const isSemver = (version: string): boolean => {
  return version.search(/^(~|\^|>=|>|<=|<|)?[0-9]+\.[0-9x]+\.[0-9x]+/) !== -1 && version.indexOf(' ') === -1;
};

export const isLooseSemver = (version: string): boolean => {
  return isSemver(version) && version.search(/\.x(\.|$)/) !== -1;
};
