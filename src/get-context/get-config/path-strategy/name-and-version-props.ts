import { O, pipe, R } from '@mobily/ts-belt';
import { isObject } from 'expect-more';
import { BaseError } from '../../../lib/error';
import { props } from '../../get-package-json-files/get-patterns/props';
import { getNonEmptyStringProp } from './lib/get-non-empty-string-prop';
import type { Entry, Strategy } from './types';

export const nameAndVersionProps: Strategy<'name~version'> = {
  read(file, pathDef) {
    return pipe(
      // get name prop
      getNonEmptyStringProp(pathDef.namePath, file),
      R.mapError(
        () =>
          new BaseError(
            `Strategy<name~version> failed to get ${pathDef.namePath} in ${file.shortPath}`,
          ),
      ),
      // add the version prop
      R.flatMap((name) =>
        pipe(
          getNonEmptyStringProp(pathDef.path, file),
          R.map((version) => ({ name, version })),
          R.mapError(
            () =>
              new BaseError(
                `Strategy<name~version> failed to get ${pathDef.path} in ${file.shortPath}`,
              ),
          ),
        ),
      ),
      // if both are non empty strings, we can return them
      R.map(({ name, version }): Entry[] => [[name, version]]),
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
        contents,
        props(pathToParent, isObject),
        O.toResult<Record<string, string | undefined>, BaseError>(onError()),
        R.tap((parent) => {
          parent[key] = version;
        }),
        R.mapError(onError),
        R.map(() => file),
      );
    } else {
      return pipe(
        R.fromExecution<void>(() => {
          contents[pathDef.path] = version;
        }),
        R.mapError(onError),
        R.map(() => file),
      );
    }

    function onError() {
      const msg = `Strategy<name~version> failed to set ${pathDef.path} in ${shortPath}`;
      return new BaseError(msg);
    }
  },
};
