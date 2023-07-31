import * as Context from '@effect/data/Context';
import type { CliConfig } from './types';

export const CliConfigTag = Context.Tag<Partial<CliConfig>>();
