import type { CliConfig, RcConfig } from './config/types';
import type { Context } from './get-context';
import { getContext } from './get-context';
import type { AnySemverGroup } from './get-semver-groups';
import { getSemverGroups } from './get-semver-groups';
import type { AnyVersionGroup } from './get-version-groups';
import { getVersionGroups } from './get-version-groups';
import { effects } from './lib/effects';

export type RcFile = Partial<RcConfig>;

export const syncpack = {
  getContext(input: Partial<CliConfig>): Context {
    return getContext(input, effects);
  },
  getSemverGroups(input: Partial<CliConfig>): AnySemverGroup[] {
    return getSemverGroups(getContext(input, effects));
  },
  getVersionGroups(input: Partial<CliConfig>): AnyVersionGroup[] {
    return getVersionGroups(getContext(input, effects));
  },
};
