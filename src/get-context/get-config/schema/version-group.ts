import { z } from 'zod';
import { baseGroupFields } from './base-group';
import { nonEmptyString } from './lib/non-empty-string';

const preferVersion = z
  .enum(['highestSemver', 'lowestSemver'])
  .optional()
  .default('highestSemver');

export const standard = z
  .object({ ...baseGroupFields, preferVersion })
  .strict();

export const banned = z
  .object({ ...baseGroupFields, isBanned: z.literal(true) })
  .strict();

export const ignored = z
  .object({ ...baseGroupFields, isIgnored: z.literal(true) })
  .strict();

export const pinned = z
  .object({ ...baseGroupFields, pinVersion: nonEmptyString })
  .strict();

export const snappedTo = z
  .object({ ...baseGroupFields, snapTo: z.array(nonEmptyString) })
  .strict();

export const defaultGroup = z
  .object({ ...baseGroupFields, isDefault: z.literal(true), preferVersion })
  .strict();

export const any = z.union([
  standard,
  banned,
  ignored,
  pinned,
  snappedTo,
  defaultGroup,
]);
