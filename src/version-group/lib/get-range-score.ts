import { RANGE } from '../../constants.js';

const scoresByRange: Record<string, number | undefined> = {
  [RANGE.ANY]: 9,
  [RANGE.WORKSPACE]: 8,
  [RANGE.GT]: 7,
  [RANGE.GTE]: 6,
  [RANGE.MINOR]: 5,
  [RANGE.LOOSE]: 4,
  [RANGE.PATCH]: 3,
  [RANGE.EXACT]: 2,
  [RANGE.LTE]: 1,
  [RANGE.LT]: 0,
};

/** Rank a Semver Range according to its greediness */
export function getRangeScore(version: string): number {
  const range =
    version.indexOf('.x') !== -1 ? RANGE.LOOSE : version.slice(0, version.search(/[0-9]/));
  return scoresByRange[range] || 0;
}
