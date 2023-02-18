import type { Disk } from '../../lib/disk';
import { verbose } from '../../lib/log';
import type { Syncpack } from '../../types';
import { getCoreTypes } from './get-core-types';
import { getCustomTypes } from './get-custom-types';
import { getEnabledTypes } from './get-enabled-types';
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
    customTypes: getConfigByName('customTypes'),
    dependencyTypes: fromRcFile?.dependencyTypes,
    filter: getConfigByName('filter'),
    indent: getConfigByName('indent'),
    semverGroups: getConfigByName('semverGroups'),
    semverRange: getConfigByName('semverRange'),
    sortAz: getConfigByName('sortAz'),
    sortFirst: getConfigByName('sortFirst'),
    source: getConfigByName('source'),
    types: fromCli?.types,
    versionGroups: getConfigByName('versionGroups'),
  });

  verbose('user config:', fromPublic);

  const coreTypes = getCoreTypes();
  const customTypes = getCustomTypes(fromPublic);
  const allTypes = [...coreTypes, ...customTypes];
  const enabledTypes = getEnabledTypes(allTypes, fromPublic);

  const allConfig = ConfigSchema.Private.parse({
    ...fromPublic,
    allTypes,
    enabledTypes,
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
    if (typeof (fromCli as any)[name] !== 'undefined')
      return (fromCli as Syncpack.Config.Public)[name];
    if (typeof (fromRcFile as any)[name] !== 'undefined')
      return (fromRcFile as Syncpack.Config.Public)[name];
  }
};
