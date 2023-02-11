import { z } from 'zod';
import { baseGroupFields } from './base-group';
import * as SemverRangeSchema from './semver-range';

export const Ignored = z
  .object({ ...baseGroupFields, isIgnored: z.literal(true) })
  .strict();

export const WithRange = z
  .object({ ...baseGroupFields, range: SemverRangeSchema.Value })
  .strict();

export const Default = z
  .object({
    ...baseGroupFields,
    range: SemverRangeSchema.Value,
    isDefault: z.literal(true),
  })
  .strict();

export const Any = z.union([Ignored, WithRange, Default]);
