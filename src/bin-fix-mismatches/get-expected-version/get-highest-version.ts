import { coerce, eq, gt, valid } from 'semver';
import { RANGE } from '../../constants';
import { isSemver } from '../../lib/is-semver';

export function getHighestVersion(versions: string[]): string {
  return versions.reduce<string>((rawHighest, raw) => {
    const version = valid(coerce(raw)) || '';
    const highest = valid(coerce(rawHighest)) || '';
    if (raw === '*' || rawHighest === '*') return '*';
    if (!isSemver(raw) || version === '') return rawHighest;
    if (highest === '') return raw;
    if (gt(version, highest)) return raw;
    if (eq(version, highest) && getRangeScore(raw) > getRangeScore(rawHighest))
      return raw;
    return rawHighest;
  }, '');
}

function getRangeScore(version: string): number {
  if (version === '') return 0;
  if (version === RANGE.ANY) return 8;
  const range = getRange(version);
  if (range === RANGE.GT) return 7;
  if (range === RANGE.GTE) return 6;
  if (range === RANGE.MINOR) return 5;
  if (version.indexOf('.x') !== -1) return 4;
  if (range === RANGE.PATCH) return 3;
  if (range === RANGE.EXACT) return 2;
  if (range === RANGE.LTE) return 1;
  if (range === RANGE.LT) return 0;
  return 0;
}

function getRange(version: string): string {
  return version.slice(0, version.search(/[0-9]/));
}
