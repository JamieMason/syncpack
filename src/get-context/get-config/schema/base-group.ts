import { z } from 'zod';
import * as DependencyTypeSchema from './dependency-type';

const NonEmptyString = z.string().trim().min(1);

export const baseGroupFields = {
  dependencies: z.array(NonEmptyString).min(1),
  dependencyTypes: DependencyTypeSchema.NameList.optional(),
  packages: z.array(NonEmptyString).min(1),
};
