import type { SemverRange } from '../config/types.js';
import { RANGE } from '../constants.js';

/** @deprecated migrate to make better use of npm-package-arg */
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
