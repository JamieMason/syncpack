import { z } from 'zod';
import { baseGroupFields } from './base-group';

const nonEmptyString = z.string().trim().min(1);

export const standard = z.object(baseGroupFields).strict();

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

export const base = z
  .object({ ...baseGroupFields, isDefault: z.literal(true) })
  .strict();

export const any = z.union([
  standard,
  banned,
  ignored,
  pinned,
  snappedTo,
  base,
]);
