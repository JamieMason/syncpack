import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { andThen } from 'tightrope/result/and-then';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import { tap } from 'tightrope/result/tap';
import { BaseError } from '../../../lib/error';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop';
import type { Entry, Strategy } from './types';

export const nameAndVersionProps: Strategy<'name~version'> = {
  read(file, pathDef) {
    return pipe(
      // get name prop
      getNonEmptyStringProp(pathDef.namePath, file),
      mapErr(
        () =>
          new BaseError(
            `Strategy<name~version> failed to get ${pathDef.namePath} in ${file.shortPath}`,
          ),
      ),
      // add the version prop
      andThen((name) =>
        pipe(
          getNonEmptyStringProp(pathDef.path, file),
          map((version) => ({ name, version })),
          mapErr(
            () =>
              new BaseError(
                `Strategy<name~version> failed to get ${pathDef.path} in ${file.shortPath}`,
              ),
          ),
        ),
      ),
      // if both are non empty strings, we can return them
      map(({ name, version }): Entry[] => [[name, version]]),
    );
  },
  write(file, pathDef, [, version]) {
    const { contents, shortPath } = file;
    const isNestedPath = pathDef.path.includes('.');

    if (isNestedPath) {
      const fullPath = pathDef.path.split('.');
      const pathToParent = fullPath.slice(0, fullPath.length - 1).join('.');
      const key = fullPath.slice(-1).join('');
      return pipe(
        get(contents, ...pathToParent.split('.')),
        tap((parent) => {
          parent[key] = version;
        }),
        mapErr(onError),
        map(() => file),
      );
    } else {
      return pipe(
        fromTry<void>(() => {
          contents[pathDef.path] = version;
        }),
        mapErr(onError),
        map(() => file),
      );
    }

    function onError() {
      const msg = `Strategy<name~version> failed to set ${pathDef.path} in ${shortPath}`;
      return new BaseError(msg);
    }
  },
};
