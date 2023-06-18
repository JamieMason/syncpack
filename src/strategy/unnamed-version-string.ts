import { pipe } from '@effect/data/Function';
import { get } from 'tightrope/fn/get';
import type { Result } from 'tightrope/result';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { tap } from 'tightrope/result/tap';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Delete } from '../get-version-groups/lib/delete';
import { DELETE } from '../get-version-groups/lib/delete';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop';

export class UnnamedVersionStringStrategy {
  _tag = 'version';
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
      map((version) => {
        const name = path.split('.').slice(-1).join('');
        return [[name, version]];
      }),
    );
  }

  write(file: PackageJsonFile, [, version]: [string, string | Delete]): Result<PackageJsonFile> {
    const path = this.path;
    const { contents } = file;
    const isNestedPath = path.includes('.');
    const nextValue = version === DELETE ? undefined : version;
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
