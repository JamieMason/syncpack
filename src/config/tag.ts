import { Context } from 'effect';
import type { CliConfig } from './types.js';

export const CliConfigTag = Context.Tag<Partial<CliConfig>>();
