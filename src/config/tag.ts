import { Context } from 'effect';
import type { CliConfig } from './types.js';

export const CliConfigTag = Context.GenericTag<Partial<CliConfig>>(
  '@services/CliConfigTag',
);
