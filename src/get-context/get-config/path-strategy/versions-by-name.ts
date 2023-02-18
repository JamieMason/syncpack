import { O, pipe, R } from '@mobily/ts-belt';
import { isNonEmptyObject } from 'expect-more/dist/is-non-empty-object';
import { isObject } from 'expect-more/dist/is-object';
import { BaseError } from '../../../lib/error';
import { props } from '../../get-package-json-files/get-patterns/props';
import type { Strategy } from './types';

export const versionsByName: Strategy<'versionsByName'> = {
  read(file, pathDef) {
    return pipe(
      file.contents,
      props(pathDef.path, isNonEmptyObject),
      O.map(Object.entries),
      O.toResult(
        new BaseError(
          `Strategy<versionsByName> failed to get ${pathDef.path} in ${file.shortPath}`,
        ),
      ),
    );
  },
  write(file, pathDef, [name, version]) {
    const { contents, shortPath } = file;

    return pipe(
      contents,
      props(pathDef.path, isObject),
      O.toResult<Record<string, string | undefined>, BaseError>(onError()),
      R.tap((parent) => {
        parent[name] = version;
      }),
      R.mapError(onError),
      R.map(() => file),
    );

    function onError() {
      const msg = `Strategy<versionsByName> failed to set ${pathDef.path} in ${shortPath}`;
      return new BaseError(msg);
    }
  },
};
