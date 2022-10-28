import { isString } from 'expect-more';
import type { ValidRange } from '../types';
import {
  RANGE_ANY,
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LOOSE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH,
} from '../constants';

export function isValidSemverRange(value: unknown): value is ValidRange {
  return (
    value === RANGE_ANY ||
    value === RANGE_EXACT ||
    value === RANGE_GT ||
    value === RANGE_GTE ||
    value === RANGE_LOOSE ||
    value === RANGE_LT ||
    value === RANGE_LTE ||
    value === RANGE_MINOR ||
    value === RANGE_PATCH
  );
}

export function isSemver(version: unknown): boolean {
  return (
    isString(version) &&
    version.search(/^(~|\^|>=|>|<=|<|)?[0-9]+\.[0-9x]+\.[0-9x]+/) !== -1 &&
    version.indexOf(' ') === -1
  );
}

export function isLooseSemver(version: unknown): boolean {
  return (
    isString(version) && isSemver(version) && version.search(/\.x(\.|$)/) !== -1
  );
}
