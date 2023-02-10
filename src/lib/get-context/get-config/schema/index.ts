import { z } from 'zod';
import { DEFAULT_CONFIG } from '../../../../constants';
import * as DependencyTypeSchema from './dependency-type';
import * as SemverGroupSchema from './semver-group';
import * as SemverRangeSchema from './semver-range';
import * as VersionGroupSchema from './version-group';

const NonEmptyString = z.string().trim().min(1);

const cliOnly = {
  configPath: z.string().optional(),
} as const;

const syncpackRcOnly = {
  semverGroups: z
    .array(SemverGroupSchema.Any)
    .default([...DEFAULT_CONFIG.semverGroups]),
  sortAz: z.array(NonEmptyString).default([...DEFAULT_CONFIG.sortAz]),
  sortFirst: z.array(NonEmptyString).default([...DEFAULT_CONFIG.sortFirst]),
  versionGroups: z
    .array(VersionGroupSchema.Any)
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
  semverRange: SemverRangeSchema.Value.default(
    DEFAULT_CONFIG.semverRange as '',
  ),
  source: z.array(NonEmptyString).default([...DEFAULT_CONFIG.source]),
} as const;

const privateOnly = {
  defaultSemverGroup: SemverGroupSchema.Default,
  defaultVersionGroup: VersionGroupSchema.Default,
  dependencyTypes: DependencyTypeSchema.NameList,
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
  defaultSemverGroup: true,
  defaultVersionGroup: true,
  dependencyTypes: true,
});
