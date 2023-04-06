import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Result } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { mapErr } from 'tightrope/result/map-err';
import { BaseError } from '../../../../lib/error';
import type { PackageJsonFile } from '../../../get-package-json-files/package-json-file';

export function getNonEmptyStringProp(
  propPath: string,
  file: PackageJsonFile,
): Result<string> {
  return pipe(
    get(file.contents, ...propPath.split('.')),
    filter(isNonEmptyString, ''),
    mapErr(
      () => new BaseError(`Failed to get ${propPath} in ${file.shortPath}`),
    ),
  );
}
