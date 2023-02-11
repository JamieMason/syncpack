import { BaseError } from '../../../lib/error';
import { nameAndVersionProps } from './name-and-version-props';
import { nameAndVersionString } from './name-and-version-string';
import { versionString } from './version-string';
import { versionsByName } from './versions-by-name';

export type StrategyByName = typeof strategyByName;

export const strategyByName = {
  'name@version': nameAndVersionString,
  'name~version': nameAndVersionProps,
  'version': versionString,
  'versionsByName': versionsByName,
} as const;

export function exhaustiveCheck(strategyName: never): never {
  throw new BaseError(`Unrecognised strategy "${strategyName}"`);
}
