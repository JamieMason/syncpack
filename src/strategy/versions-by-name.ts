import { Effect, Option, pipe } from 'effect';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object.js';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file.js';
import { get } from '../lib/get.js';
import type { Delete } from '../version-group/lib/delete.js';
import { DELETE } from '../version-group/lib/delete.js';

const getOptionOfNonEmptyObject = Option.liftPredicate(isNonEmptyObject<any>);

export class VersionsByNameStrategy {
  _tag = 'versionsByName';
  name: string;
  path: string;

  constructor(name: string, path: string) {
    this.name = name;
    this.path = path;
  }

  read(file: PackageJsonFile): Effect.Effect<never, never, [string, string][]> {
    return pipe(
      get(file.jsonFile.contents, ...this.path.split('.')),
      Effect.flatMap((value) => getOptionOfNonEmptyObject(value)),
      Effect.map((obj) => Object.entries<string>(obj)),
      Effect.tapError(() =>
        Effect.logDebug(
          `VersionsByNameStrategy#${this.name} found nothing at <${file.jsonFile.shortPath}>.${this.path}`,
        ),
      ),
      // if value is invalid, default to empty
      Effect.catchAll(() => Effect.succeed([])),
    );
  }

  write(
    file: PackageJsonFile,
    [name, version]: [string, string | Delete],
  ): Effect.Effect<never, never, PackageJsonFile> {
    const nextValue = version === DELETE ? undefined : version;
    return pipe(
      get(file.jsonFile.contents, ...this.path.split('.')),
      Effect.flatMap(getOptionOfNonEmptyObject),
      Effect.flatMap((parent) =>
        Effect.try(() => {
          parent[name] = nextValue;
        }),
      ),
      Effect.tapError(() =>
        Effect.logDebug(
          `strategy ${this._tag} with name ${this.name} failed to write to <${file.jsonFile.shortPath}>.${this.path}.${name}`,
        ),
      ),
      Effect.catchAll(() => Effect.succeed(file)),
      Effect.map(() => file),
    );
  }
}
