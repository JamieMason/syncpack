import { z } from 'zod';
import { baseGroupFields } from './base-group';

const NonEmptyString = z.string().trim().min(1);

export const Standard = z.object(baseGroupFields).strict();

export const Banned = z
  .object({ ...baseGroupFields, isBanned: z.literal(true) })
  .strict();

export const Ignored = z
  .object({ ...baseGroupFields, isIgnored: z.literal(true) })
  .strict();

export const Pinned = z
  .object({ ...baseGroupFields, pinVersion: NonEmptyString })
  .strict();

export const Default = z
  .object({ ...baseGroupFields, isDefault: z.literal(true) })
  .strict();

export const Any = z.union([Standard, Banned, Ignored, Pinned, Default]);
