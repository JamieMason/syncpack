import { pipe } from '@effect/data/Function';
import { get } from 'tightrope/fn/get';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Result } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { mapErr } from 'tightrope/result/map-err';
import type { PackageJsonFile } from '../../get-package-json-files/package-json-file';

export function getNonEmptyStringProp(propPath: string, file: PackageJsonFile): Result<string> {
  return pipe(
    get(file.jsonFile.contents, ...propPath.split('.')),
    filter(isNonEmptyString, ''),
    mapErr(() => new Error(`Failed to get ${propPath} in ${file.jsonFile.shortPath}`)),
  );
}
