import { isString } from 'expect-more';
import { RANGE } from '../constants';
import type { Syncpack } from '../types';

export function isValidSemverRange(
  value: unknown,
): value is Syncpack.Config.SemverRange.Value {
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

export function isSemver(version: unknown): version is string {
  const range = '(~|\\^|>=|>|<=|<)?';
  const ints = '[0-9]+';
  const intsOrX = '([0-9]+|x)';
  const dot = '\\.';
  const major = new RegExp(`^${range}${ints}$`);
  const minor = new RegExp(`^${range}${ints}${dot}${intsOrX}$`);
  const patch = new RegExp(`^${range}${ints}${dot}${intsOrX}${dot}${intsOrX}$`);
  return (
    isString(version) &&
    (version.search(major) !== -1 ||
      version.search(minor) !== -1 ||
      version.search(patch) !== -1)
  );
}

export function isLooseSemver(version: unknown): boolean {
  return isSemver(version) && version.search(/\.x(\.|$)/) !== -1;
}
