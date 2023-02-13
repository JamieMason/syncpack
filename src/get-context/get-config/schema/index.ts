import { z } from 'zod';
import { DEFAULT_CONFIG } from '../../../constants';
import * as paths from './paths';
import * as semverGroup from './semver-group';
import * as semverRange from './semver-range';
import * as versionGroup from './version-group';

const nonEmptyString = z.string().trim().min(1);

const cliOnly = {
  configPath: z.string().optional(),
  types: z.string().default(''),
} as const;

const syncpackRcOnly = {
  customTypes: paths.pathConfigByName.optional(),
  dependencyTypes: z.array(nonEmptyString).default([]),
  semverGroups: z
    .array(semverGroup.any)
    .default([...DEFAULT_CONFIG.semverGroups]),
  sortAz: z.array(nonEmptyString).default([...DEFAULT_CONFIG.sortAz]),
  sortFirst: z.array(nonEmptyString).default([...DEFAULT_CONFIG.sortFirst]),
  versionGroups: z
    .array(versionGroup.any)
    .default([...DEFAULT_CONFIG.versionGroups]),
} as const;

const cliAndRcFile = {
  filter: nonEmptyString.default(DEFAULT_CONFIG.filter),
  indent: z.string().default(DEFAULT_CONFIG.indent),
  semverRange: semverRange.value.default(DEFAULT_CONFIG.semverRange as ''),
  source: z.array(nonEmptyString).default([...DEFAULT_CONFIG.source]),
} as const;

const privateOnly = {
  allTypes: z.array(paths.pathDefinition),
  enabledTypes: z.array(paths.pathDefinition),
  defaultSemverGroup: semverGroup.base,
  defaultVersionGroup: versionGroup.base,
} as const;

export const Private = z.object({
  ...privateOnly,
  ...cliOnly,
  ...syncpackRcOnly,
  ...cliAndRcFile,
});

export const SyncpackRc = z.object({
  ...syncpackRcOnly,
  ...cliAndRcFile,
});

export const Cli = z.object({
  ...cliOnly,
  ...cliAndRcFile,
});

export const Public = Private.omit({
  allTypes: true,
  enabledTypes: true,
  defaultSemverGroup: true,
  defaultVersionGroup: true,
});
