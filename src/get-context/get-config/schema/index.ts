import { z } from 'zod';
import { DEFAULT_CONFIG } from '../../../constants';
import * as paths from './paths';
import * as semverGroup from './semver-group';
import * as semverRange from './semver-range';
import * as versionGroup from './version-group';

const NonEmptyString = z.string().trim().min(1);

const cliOnly = {
  configPath: z.string().optional(),
} as const;

const syncpackRcOnly = {
  customPaths: paths.pathConfigByName.optional(),
  semverGroups: z
    .array(semverGroup.any)
    .default([...DEFAULT_CONFIG.semverGroups]),
  sortAz: z.array(NonEmptyString).default([...DEFAULT_CONFIG.sortAz]),
  sortFirst: z.array(NonEmptyString).default([...DEFAULT_CONFIG.sortFirst]),
  versionGroups: z
    .array(versionGroup.any)
    .default([...DEFAULT_CONFIG.versionGroups]),
} as const;

const cliAndRcFile = {
  dev: z.boolean().default(DEFAULT_CONFIG.dev),
  overrides: z.boolean().default(DEFAULT_CONFIG.overrides),
  peer: z.boolean().default(DEFAULT_CONFIG.peer),
  pnpmOverrides: z.boolean().default(DEFAULT_CONFIG.pnpmOverrides),
  prod: z.boolean().default(DEFAULT_CONFIG.prod),
  resolutions: z.boolean().default(DEFAULT_CONFIG.resolutions),
  workspace: z.boolean().default(DEFAULT_CONFIG.workspace),
  filter: NonEmptyString.default(DEFAULT_CONFIG.filter),
  indent: z.string().default(DEFAULT_CONFIG.indent),
  semverRange: semverRange.value.default(DEFAULT_CONFIG.semverRange as ''),
  source: z.array(NonEmptyString).default([...DEFAULT_CONFIG.source]),
} as const;

const privateOnly = {
  corePaths: z.array(paths.pathDefinition),
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
  corePaths: true,
  defaultSemverGroup: true,
  defaultVersionGroup: true,
});
