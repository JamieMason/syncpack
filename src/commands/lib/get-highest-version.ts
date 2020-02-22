import { coerce, eq, gt, valid } from 'semver';
import {
  RANGE_ANY,
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH,
} from '../../constants';

const getRange = (version: string): string => version.slice(0, version.search(/[0-9]/));

const getRangeScore = (version: string | null): number => {
  if (version === null) return 0;
  if (version === RANGE_ANY) return 8;
  const range = getRange(version);
  if (range === RANGE_GT) return 7;
  if (range === RANGE_GTE) return 6;
  if (range === RANGE_MINOR) return 5;
  if (version.indexOf('.x') !== -1) return 4;
  if (range === RANGE_PATCH) return 3;
  if (range === RANGE_EXACT) return 2;
  if (range === RANGE_LTE) return 1;
  if (range === RANGE_LT) return 0;
  return 0;
};

export const getHighestVersion = (versions: string[]): string => {
  let rawHighest: string | null = null;
  for (const raw of versions) {
    if (raw === '*') return raw;
    const version = valid(coerce(raw));
    if (version === null) continue;
    const highest = valid(coerce(rawHighest));
    if (
      highest === null ||
      gt(version, highest) ||
      (eq(version, highest) && getRangeScore(raw) > getRangeScore(rawHighest))
    ) {
      rawHighest = raw;
    }
  }
  if (rawHighest === null) {
    throw new Error(`Failed to find highest version of ${versions.join(', ')}`);
  }
  return rawHighest;
};
