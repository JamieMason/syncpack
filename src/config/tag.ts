import { Context } from 'effect';
import type { CliConfig } from './types';

export const CliConfigTag = Context.Tag<Partial<CliConfig>>();
