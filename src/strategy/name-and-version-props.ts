import { Effect, identity, Option, pipe } from 'effect';
import { isObject } from 'tightrope/guard/is-object.js';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file.js';
import { get } from '../lib/get.js';
import type { Delete } from '../version-group/lib/delete.js';
import { DELETE } from '../version-group/lib/delete.js';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop.js';

const getOptionOfObject = Option.liftPredicate(isObject<any>);

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

  read(file: PackageJsonFile): Effect.Effect<never, never, [string, string][]> {
    return pipe(
      Effect.Do,
      // get the name prop
      Effect.bind('name', () => getNonEmptyStringProp(this.namePath, file)),
      // add the version prop
      Effect.bind('version', () =>
        pipe(
          getNonEmptyStringProp(this.path, file),
          /**
           * In order to report a `MissingLocalVersion`, we need to ensure that
           * a value is returned for `local` package .version properties so we
           * can know that `this.name` is a package developed in this repo but
           * that its version is missing.
           *
           * Not doing this results in the invalid local package being ignored
           * and each installation of it being checked for mismatches amongst
           * themselves.
           */
          this.name === 'local'
            ? Effect.catchAll(() => Effect.succeed('PACKAGE_JSON_HAS_NO_VERSION'))
            : Effect.map(identity),
        ),
      ),
      // if both are non empty strings, we can return them
      Effect.map(({ name, version }): [string, string][] => [[name, version]]),
      Effect.tapError(() =>
        Effect.logDebug(
          `NameAndVersionPropsStrategy#${this.name} found nothing at <${file.jsonFile.shortPath}>.${this.path} & .${this.namePath}`,
        ),
      ),
      // if either are invalid, default to empty
      Effect.catchAll(() => Effect.succeed([])),
    );
  }

  write(
    file: PackageJsonFile,
    [, version]: [string, string | Delete],
  ): Effect.Effect<never, never, PackageJsonFile> {
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
        Effect.flatMap(getOptionOfObject),
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
        getOptionOfObject(contents),
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
