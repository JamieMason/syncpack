import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Result } from 'tightrope/result';
import { Err, Ok } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { tap } from 'tightrope/result/tap';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Delete } from '../get-version-groups/lib/delete';
import { DELETE } from '../get-version-groups/lib/delete';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop';

export class NamedVersionStringStrategy {
  _tag = 'name@version';
  name: string;
  path: string;

  constructor(name: string, path: string) {
    this.name = name;
    this.path = path;
  }

  read(file: PackageJsonFile): Result<[string, string][]> {
    const path = this.path;
    return pipe(
      // get version prop
      getNonEmptyStringProp(path, file),
      // if it is a non empty string, we can read it
      andThen((value) => {
        const [name, version] = value.split(/@(.*)/);
        return isNonEmptyString(name) && isNonEmptyString(version)
          ? new Ok<[string, string][]>([[name, version]])
          : new Err(
              new Error(
                `Strategy<name@version> failed to get ${path} in ${file.shortPath}`,
              ),
            );
      }),
    );
  }

  write(
    file: PackageJsonFile,
    [name, version]: [string, string | Delete],
  ): Result<PackageJsonFile> {
    const { contents } = file;
    const path = this.path;
    const isNestedPath = path.includes('.');
    const nextValue = version === DELETE ? undefined : `${name}@${version}`;
    if (isNestedPath) {
      const fullPath = path.split('.');
      const pathToParent = fullPath.slice(0, fullPath.length - 1).join('.');
      const key = fullPath.slice(-1).join('');
      return pipe(
        get(contents, ...pathToParent.split('.')),
        tap((parent) => {
          parent[key] = nextValue;
        }),
        map(() => file),
      );
    } else {
      return pipe(
        fromTry<void>(() => {
          contents[path] = nextValue;
        }),
        map(() => file),
      );
    }
  }
}
