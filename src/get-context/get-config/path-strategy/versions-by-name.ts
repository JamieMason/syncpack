import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import { filter } from 'tightrope/result/filter';
import { map as mapR } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import { tap } from 'tightrope/result/tap';
import { BaseError } from '../../../lib/error';
import type { Strategy } from './types';

export const versionsByName: Strategy<'versionsByName'> = {
  read(file, pathDef) {
    return pipe(
      get(file.contents, ...pathDef.path.split('.')),
      filter(isNonEmptyObject<string>, ''),
      mapR(Object.entries<string>),
      mapErr(
        () =>
          new BaseError(
            `Strategy<versionsByName> failed to get ${pathDef.path} in ${file.shortPath}`,
          ),
      ),
    );
  },
  write(file, pathDef, [name, version]) {
    return pipe(
      get(file.contents, ...pathDef.path.split('.')),
      tap((parent) => {
        parent[name] = version;
      }),
      mapErr(
        () =>
          new BaseError(
            `Strategy<versionsByName> failed to set ${pathDef.path} in ${file.shortPath}`,
          ),
      ),
      mapR(() => file),
    );
  },
};
