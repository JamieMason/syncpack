import { isString } from 'expect-more';
import type { ValidRange } from '../types';
import { RANGE } from '../constants';

export function isValidSemverRange(value: unknown): value is ValidRange {
  return (
    value === RANGE.ANY ||
    value === RANGE.EXACT ||
    value === RANGE.GT ||
    value === RANGE.GTE ||
    value === RANGE.LOOSE ||
    value === RANGE.LT ||
    value === RANGE.LTE ||
    value === RANGE.MINOR ||
    value === RANGE.PATCH
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
