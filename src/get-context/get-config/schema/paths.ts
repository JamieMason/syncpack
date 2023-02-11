import { z } from 'zod';

const namedVersionString = z.object({
  path: z.string(),
  strategy: z.literal('name@version'),
});

const nameAndVersionStrings = z.object({
  namePath: z.string(),
  path: z.string(),
  strategy: z.literal('name~version'),
});

const unnamedVersionString = z.object({
  path: z.string(),
  strategy: z.literal('version'),
});

const versionsByName = z.object({
  path: z.string(),
  strategy: z.literal('versionsByName'),
});

const pathConfig = z.discriminatedUnion('strategy', [
  nameAndVersionStrings,
  namedVersionString,
  unnamedVersionString,
  versionsByName,
]);

/** config */
export const pathConfigByName = z.record(pathConfig);

/** private */
export const pathDefinition = pathConfig.and(
  z.object({
    name: z.string().trim().min(1),
  }),
);
