import { z } from 'zod';
import { ALL_DEPENDENCY_TYPES, DEFAULT_CONFIG } from '../../../constants';

export const Name = z.enum(ALL_DEPENDENCY_TYPES);

export const NameList = z.array(Name);

export const Flags = z.object({
  dev: z.boolean().default(DEFAULT_CONFIG.dev),
  overrides: z.boolean().default(DEFAULT_CONFIG.overrides),
  peer: z.boolean().default(DEFAULT_CONFIG.peer),
  pnpmOverrides: z.boolean().default(DEFAULT_CONFIG.pnpmOverrides),
  prod: z.boolean().default(DEFAULT_CONFIG.prod),
  resolutions: z.boolean().default(DEFAULT_CONFIG.resolutions),
  workspace: z.boolean().default(DEFAULT_CONFIG.workspace),
});
