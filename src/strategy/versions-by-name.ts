import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import type { Result } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { map as mapR } from 'tightrope/result/map';
import { tap } from 'tightrope/result/tap';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import type { Delete } from '../get-version-groups/lib/delete';
import { DELETE } from '../get-version-groups/lib/delete';

export class VersionsByNameStrategy {
  _tag = 'versionsByName';
  name: string;
  path: string;

  constructor(name: string, path: string) {
    this.name = name;
    this.path = path;
  }

  read(file: PackageJsonFile): Result<[string, string][]> {
    const path = this.path;
    return pipe(
      get(file.contents, ...path.split('.')),
      filter(isNonEmptyObject<string>, ''),
      mapR(Object.entries<string>),
    );
  }

  write(
    file: PackageJsonFile,
    [name, version]: [string, string | Delete],
  ): Result<PackageJsonFile> {
    const path = this.path;
    const nextValue = version === DELETE ? undefined : version;
    return pipe(
      get(file.contents, ...path.split('.')),
      tap((parent) => {
        parent[name] = nextValue;
      }),
      mapR(() => file),
    );
  }
}
