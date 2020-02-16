import {
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LOOSE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH,
} from '../../constants';

export const isValidSemverRange = (range: string) =>
  range === RANGE_EXACT ||
  range === RANGE_GT ||
  range === RANGE_GTE ||
  range === RANGE_LOOSE ||
  range === RANGE_LT ||
  range === RANGE_LTE ||
  range === RANGE_MINOR ||
  range === RANGE_PATCH;

export const isSemver = (version: string) => {
  return version.search(/^(~|\^|>=|>|<=|<|)?[0-9]+\.[0-9x]+\.[0-9x]+/) !== -1 && version.indexOf(' ') === -1;
};

export const isLooseSemver = (version: string) => {
  return isSemver(version) && version.search(/\.x(\.|$)/) !== -1;
};
