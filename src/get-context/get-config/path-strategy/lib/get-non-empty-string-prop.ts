import type { R } from '@mobily/ts-belt';
import { O, pipe } from '@mobily/ts-belt';
import { isNonEmptyString } from 'expect-more';
import { BaseError } from '../../../../lib/error';
import { props } from '../../../get-package-json-files/get-patterns/props';
import type { PackageJsonFile } from '../../../get-package-json-files/package-json-file';

// const root: any = this.packageJsonFile.contents;
// if (this.pathDef.name === 'pnpmOverrides') {
//   root.pnpm.overrides[this.name] = version;
// } else if (this.pathDef.name !== 'workspace') {
//   root[(this.pathDef as any).path][this.name] = version;
// }

export function getNonEmptyStringProp(
  propPath: string,
  file: PackageJsonFile,
): R.Result<string, BaseError> {
  return pipe(
    file.contents,
    props(propPath, isNonEmptyString),
    O.toResult<string, BaseError>(
      new BaseError(`Failed to get ${propPath} in ${file.shortPath}`),
    ),
  );
}
