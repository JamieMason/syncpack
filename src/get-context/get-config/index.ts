import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfObjects } from 'tightrope/guard/is-array-of-objects';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isString } from 'tightrope/guard/is-string';
import type { Result } from 'tightrope/result';
import { fromTry } from 'tightrope/result/from-try';
import { mapErr } from 'tightrope/result/map-err';
import type { Disk } from '../../lib/disk';
import { BaseError } from '../../lib/error';
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
export function getConfig(
  disk: Disk,
  fromCli: Partial<Syncpack.Config.Cli>,
): Result<Syncpack.Config.Private> {
  const ERR_READING_CONFIG = 'Error reading config';
  return pipe(
    fromTry(() => unSafeGetConfig(disk, fromCli)),
    mapErr(BaseError.map(ERR_READING_CONFIG)),
  );
}

function unSafeGetConfig(
  disk: Disk,
  fromCli: Partial<Syncpack.Config.Cli>,
): Syncpack.Config.Private {
  verbose('cli arguments:', fromCli);

  const fromRcFile = disk.readConfigFileSync(fromCli.configPath);

  verbose('rcfile contents:', fromRcFile);

  const fromPublic = ConfigSchema.Public.parse({
    customTypes: getConfigByName('customTypes', isNonEmptyObject),
    dependencyTypes: fromRcFile?.dependencyTypes,
    filter: getConfigByName('filter', isNonEmptyString),
    indent: getConfigByName('indent', isString),
    semverGroups: getConfigByName('semverGroups', isArrayOfObjects),
    semverRange: getConfigByName('semverRange', isString),
    sortAz: getConfigByName('sortAz', isArrayOfStrings),
    sortFirst: getConfigByName('sortFirst', isArrayOfStrings),
    source: getConfigByName('source', isArrayOfStrings),
    types: fromCli?.types,
    versionGroups: getConfigByName('versionGroups', isArrayOfObjects),
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

  function getConfigByName(
    name: keyof Syncpack.Config.Public,
    isValid: (value: unknown) => boolean,
  ): unknown {
    if (isValid((fromCli as any)[name]))
      return (fromCli as Syncpack.Config.Public)[name];
    if (isValid((fromRcFile as any)[name]))
      return (fromRcFile as Syncpack.Config.Public)[name];
  }
}
