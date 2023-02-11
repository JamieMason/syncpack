import type { Disk } from '../../lib/disk';
import { verbose } from '../../lib/log';
import type { Syncpack } from '../../types';
import { getCorePaths } from './get-core-paths';
import * as ConfigSchema from './schema';

/**
 * Take all configuration from the command line and config file, combine it, and
 * set defaults for anything which hasn't been defined.
 */
export const getConfig = (
  disk: Disk,
  fromCli: Partial<Syncpack.Config.Cli>,
): Syncpack.Config.Private => {
  verbose('cli arguments:', fromCli);

  const fromRcFile = disk.readConfigFileSync(fromCli.configPath);

  verbose('rcfile contents:', fromRcFile);

  const fromPublic = ConfigSchema.Public.parse({
    customPaths: getConfigByName('customPaths'),
    dev: getConfigByName('dev'),
    filter: getConfigByName('filter'),
    indent: getConfigByName('indent'),
    overrides: getConfigByName('overrides'),
    peer: getConfigByName('peer'),
    pnpmOverrides: getConfigByName('pnpmOverrides'),
    prod: getConfigByName('prod'),
    resolutions: getConfigByName('resolutions'),
    semverGroups: getConfigByName('semverGroups'),
    semverRange: getConfigByName('semverRange'),
    sortAz: getConfigByName('sortAz'),
    sortFirst: getConfigByName('sortFirst'),
    source: getConfigByName('source'),
    versionGroups: getConfigByName('versionGroups'),
    workspace: getConfigByName('workspace'),
  });

  verbose('user config:', fromPublic);

  const allConfig = ConfigSchema.Private.parse({
    ...fromPublic,
    corePaths: getCorePaths(fromCli),
    defaultSemverGroup: {
      dependencies: ['**'],
      isDefault: true,
      packages: ['**'],
      range: fromPublic.semverRange,
    },
    defaultVersionGroup: {
      dependencies: ['**'],
      isDefault: true,
      packages: ['**'],
    },
  });

  allConfig.semverGroups.push(allConfig.defaultSemverGroup);
  allConfig.versionGroups.push(allConfig.defaultVersionGroup);

  verbose('final config:', allConfig);

  return allConfig;

  function getConfigByName(name: keyof Syncpack.Config.Public): unknown {
    if (name in fromCli) return (fromCli as Syncpack.Config.Public)[name];
    if (name in fromRcFile) return (fromRcFile as Syncpack.Config.Public)[name];
  }
};
