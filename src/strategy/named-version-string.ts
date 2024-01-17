import { Effect, Option, pipe } from 'effect';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object.js';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file.js';
import { get } from '../lib/get.js';
import type { Delete } from '../version-group/lib/delete.js';
import { DELETE } from '../version-group/lib/delete.js';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop.js';

const getOptionOfNonEmptyString = Option.liftPredicate(isNonEmptyString);
const getOptionOfNonEmptyObject = Option.liftPredicate(isNonEmptyObject<any>);

export class NamedVersionStringStrategy {
  _tag = 'name@version';
  name: string;
  path: string;

  constructor(name: string, path: string) {
    this.name = name;
    this.path = path;
  }

  read(file: PackageJsonFile): Effect.Effect<never, never, [string, string][]> {
    const path = this.path;
    return pipe(
      // get version prop
      getNonEmptyStringProp(path, file),
      // if it is a non empty string, we can read it
      Effect.map((value) => value.split(/@(.*)/)),
      // check the string was properly formed
      Effect.flatMap(([name, version]) =>
        Effect.all([getOptionOfNonEmptyString(name), getOptionOfNonEmptyString(version)]),
      ),
      // return an array of one entry if valid
      Effect.map(([name, version]): [string, string][] => [[name, version]]),
      Effect.tapError(() =>
        Effect.logDebug(
          `NamedVersionStringStrategy#${this.name} found nothing at <${file.jsonFile.shortPath}>.${this.path}`,
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
    const { contents } = file.jsonFile;
    const isNestedPath = this.path.includes('.');
    const nextValue = version === DELETE ? undefined : `${name}@${version}`;
    if (isNestedPath) {
      const fullPath = this.path.split('.');
      const pathToParent = fullPath.slice(0, fullPath.length - 1).join('.');
      const key = fullPath.slice(-1).join('');
      return pipe(
        get(contents, ...pathToParent.split('.')),
        Effect.flatMap(getOptionOfNonEmptyObject),
        Effect.flatMap((parent) =>
          Effect.try(() => {
            parent[key] = nextValue;
          }),
        ),
        Effect.tapError(() =>
          Effect.logDebug(
            `strategy ${this._tag} with name ${this.name} failed to write to <${file.jsonFile.shortPath}>.${this.path}`,
          ),
        ),
        Effect.catchAll(() => Effect.succeed(file)),
        Effect.map(() => file),
      );
    } else {
      return pipe(
        getOptionOfNonEmptyObject(contents),
        Effect.flatMap((parent) =>
          Effect.try(() => {
            parent[this.path] = nextValue;
          }),
        ),
        Effect.tapError(() =>
          Effect.logDebug(
            `strategy ${this._tag} with name ${this.name} failed to write to <${file.jsonFile.shortPath}>.${this.path}`,
          ),
        ),
        Effect.catchAll(() => Effect.succeed(file)),
        Effect.map(() => file),
      );
    }
  }
}
