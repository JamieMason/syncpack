import { z } from 'zod';
import { baseGroupFields } from './base-group';
import * as SemverRangeSchema from './semver-range';

export const ignored = z
  .object({ ...baseGroupFields, isIgnored: z.literal(true) })
  .strict();

export const withRange = z
  .object({ ...baseGroupFields, range: SemverRangeSchema.value })
  .strict();

export const base = z
  .object({
    ...baseGroupFields,
    range: SemverRangeSchema.value,
    isDefault: z.literal(true),
  })
  .strict();

export const any = z.union([ignored, withRange, base]);
