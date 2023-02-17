import { z } from 'zod';

export const baseGroupFields = {
  dependencies: z.array(z.string()).min(1),
  dependencyTypes: z.array(z.string()).default([]),
  label: z.string().default(''),
  packages: z.array(z.string()).min(1),
};
