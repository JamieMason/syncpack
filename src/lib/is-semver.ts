import { isString } from 'tightrope/guard/is-string';
import type { SemverRange } from '../config/types';
import { RANGE } from '../constants';

export function isValidSemverRange(value: unknown): value is SemverRange {
  return (
    value === RANGE.ANY ||
    value === RANGE.EXACT ||
    value === RANGE.GT ||
    value === RANGE.GTE ||
    value === RANGE.LOOSE ||
    value === RANGE.LT ||
    value === RANGE.LTE ||
    value === RANGE.MINOR ||
    value === RANGE.PATCH ||
    value === RANGE.WORKSPACE
  );
}

export function isSupported(version: unknown): version is string {
  return version === '*' || isSemver(version) || isWorkspaceProtocol(version);
}

export function isWorkspaceProtocol(version: unknown): boolean {
  if (!isString(version)) return false;
  if (!version.startsWith('workspace:')) return false;
  const value = version.replace(/^workspace:/, '');
  return value === '*' || isSemver(value);
}

export function isSemver(version: unknown): boolean {
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
  return (
    isString(version) && isSemver(version) && version.search(/\.x(\.|$)/) !== -1
  );
}
