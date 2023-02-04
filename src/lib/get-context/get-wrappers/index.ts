import * as E from 'fp-ts/lib/Either';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import type { Disk } from '../../disk';
import type { InternalConfig } from '../get-config/internal-config';
import { getFilePaths } from './get-file-paths';
import { readJsonSafe } from './get-patterns/read-json-safe';
import { Wrapper } from './wrapper';

export interface Source {
  bugs?: { url: string } | string;
  dependencies?: Record<string, string>;
  description?: string;
  devDependencies?: Record<string, string>;
  keywords?: string[];
  name?: string;
  overrides?: Record<string, string>;
  peerDependencies?: Record<string, string>;
  pnpm?: {
    overrides?: Record<string, string>;
  };
  repository?: { directory?: string; type: string; url: string } | string;
  resolutions?: Record<string, string>;
  scripts?: Record<string, string>;
  version?: string;
  workspaces?: Record<string, string[]> | string[];
  [otherProps: string]:
    | Record<string, string | string[] | Record<string, string | string[]>>
    | string
    | string[]
    | undefined;
}

/** Create an API for every package.json file needed. */
export function getWrappers(disk: Disk, program: InternalConfig): Wrapper[] {
  const useEmpty = () => [];
  return pipe(
    getFilePaths(disk, program),
    E.chain(
      flow(
        O.getOrElse(useEmpty as () => string[]),
        E.traverseArray(readJsonSafe<Source>(disk)),
        E.map((wrappers) =>
          wrappers.map((wrapper) => new Wrapper(wrapper, program, disk)),
        ),
      ),
    ),
    E.fold(useEmpty as () => Wrapper[], (wrappers) => wrappers),
  );
}
