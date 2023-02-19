import { z } from 'zod';

export const baseGroupFields = {
  dependencies: z.array(z.string()).min(1),
  dependencyTypes: z.array(z.string()).default([]).optional(),
  label: z.string().default('').optional(),
  packages: z.array(z.string()).min(1),
};
