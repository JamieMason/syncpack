import { z } from 'zod';

const nonEmptyString = z.string().trim().min(1);

export const baseGroupFields = {
  dependencies: z.array(nonEmptyString).min(1),
  dependencyTypes: z.array(nonEmptyString).default([]),
  packages: z.array(nonEmptyString).min(1),
};
