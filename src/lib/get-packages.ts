import fs = require('fs-extra');
import globby = require('globby');
import { OPTION_SOURCES } from '../constants';
import { CommanderApi, IManifestDescriptor } from '../typings';

export const getSources = (program: CommanderApi): string[] =>
  program.source && program.source.length
    ? program.source
    : OPTION_SOURCES.default;

export const getPackages = async (
  program: CommanderApi
): Promise<IManifestDescriptor[]> =>
  Promise.all(
    (await globby(getSources(program))).map(async (filePath) => ({
      data: await fs.readJSON(filePath),
      path: filePath
    }))
  );
