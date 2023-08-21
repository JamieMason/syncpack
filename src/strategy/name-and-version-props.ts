import { pipe } from '@effect/data/Function';
import { get } from 'tightrope/fn/get';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { tap } from 'tightrope/result/tap';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Delete } from '../get-version-groups/lib/delete';
import { DELETE } from '../get-version-groups/lib/delete';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop';

export class NameAndVersionPropsStrategy {
  _tag = 'name~version';
  name: string;
  path: string;
  namePath: string;

  constructor(name: string, path: string, namePath: string) {
    this.name = name;
    this.path = path;
    this.namePath = namePath;
  }

  read(file: PackageJsonFile): Result<[string, string][]> {
    const path = this.path;
    const namePath = this.namePath;
    return pipe(
      // get name prop
      getNonEmptyStringProp(namePath, file),
      // add the version prop
      andThen((name) =>
        pipe(
          getNonEmptyStringProp(path, file),
          map((version) => ({ name, version })),
        ),
      ),
      // if both are non empty strings, we can return them
      map(({ name, version }): [string, string][] => [[name, version]]),
    );
  }

  write(file: PackageJsonFile, [, version]: [string, string | Delete]): Result<PackageJsonFile> {
    const path = this.path;
    const { contents } = file.jsonFile;
    const isNestedPath = path.includes('.');
    const nextValue = version === DELETE ? undefined : version;

    if (isNestedPath) {
      const fullPath = path.split('.');
      const pathToParent = fullPath.slice(0, fullPath.length - 1).join('.');
      const key = fullPath.slice(-1).join('');
      return pipe(
        get(contents, ...pathToParent.split('.')),
        tap((parent) => {
          parent[key] = version;
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
