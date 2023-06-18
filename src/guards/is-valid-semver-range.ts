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
