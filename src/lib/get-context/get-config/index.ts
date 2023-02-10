import type { TConfig } from '../../../types';
import type { Disk } from '../../disk';
import { verbose } from '../../log';
import { getDependencyTypes } from './get-dependency-types';
import * as ConfigSchema from './schema';

/**
 * Take all configuration from the command line and config file, combine it, and
 * set defaults for anything which hasn't been defined.
 */
export const getConfig = (
  disk: Disk,
  fromCli: Partial<TConfig.Cli>,
): TConfig.Private => {
  verbose('cli arguments:', fromCli);

  const fromRcFile = disk.readConfigFileSync(fromCli.configPath);

  verbose('rcfile contents:', fromCli);

  const fromPublic = ConfigSchema.Public.parse({
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

  const allConfig = ConfigSchema.Private.parse({
    ...fromPublic,
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
    dependencyTypes: getDependencyTypes(fromCli, fromPublic),
  });

  allConfig.semverGroups.push(allConfig.defaultSemverGroup);
  allConfig.versionGroups.push(allConfig.defaultVersionGroup);

  verbose('final config:', allConfig);

  return allConfig;

  function getConfigByName(name: keyof TConfig.Public): unknown {
    if (name in fromCli) return (fromCli as TConfig.Public)[name];
    if (name in fromRcFile) return (fromRcFile as TConfig.Public)[name];
  }
};
