import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { Err, Ok } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import { tap } from 'tightrope/result/tap';
import { BaseError } from '../../../lib/error';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop';
import type { Entry, Strategy } from './types';

export const nameAndVersionString: Strategy<'name@version'> = {
  read(file, pathDef) {
    return pipe(
      // get version prop
      getNonEmptyStringProp(pathDef.path, file),
      // if it is a non empty string, we can read it
      andThen((value) => {
        const [name, version] = value.split('@');
        return isNonEmptyString(name) && isNonEmptyString(version)
          ? new Ok<Entry[]>([[name, version]])
          : new Err(
              new BaseError(
                `Strategy<name@version> failed to get ${pathDef.path} in ${file.shortPath}`,
              ),
            );
      }),
    );
  },
  write(file, pathDef, [name, version]) {
    const { contents, shortPath } = file;
    const isNestedPath = pathDef.path.includes('.');
    if (isNestedPath) {
      const fullPath = pathDef.path.split('.');
      const pathToParent = fullPath.slice(0, fullPath.length - 1).join('.');
      const key = fullPath.slice(-1).join('');
      return pipe(
        get(contents, ...pathToParent.split('.')),
        tap((parent) => {
          parent[key] = `${name}@${version}`;
        }),
        mapErr(onError),
        map(() => file),
      );
    } else {
      return pipe(
        fromTry<void>(() => {
          contents[pathDef.path] = `${name}@${version}`;
        }),
        mapErr(onError),
        map(() => file),
      );
    }

    function onError() {
      const msg = `Strategy<name@version> failed to set ${pathDef.path} in ${shortPath}`;
      return new BaseError(msg);
    }
  },
};
