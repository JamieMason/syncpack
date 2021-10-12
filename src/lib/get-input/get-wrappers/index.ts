import * as E from 'fp-ts/lib/Either';
import * as O from 'fp-ts/lib/Option';
import { flow, pipe } from 'fp-ts/lib/function';
import type { SyncpackConfig } from '../../../constants';
import { getFilePaths } from './get-file-paths';
import { readJsonSafe } from './get-patterns/read-json-safe';
import { removeReadonlyType } from './readonly';
import type { Disk } from '../../../lib/disk';

export interface Source {
  bugs?: { url: string } | string;
  dependencies?: Record<string, string>;
  description?: string;
  devDependencies?: Record<string, string>;
  keywords?: string[];
  name?: string;
  peerDependencies?: Record<string, string>;
  repository?: { type: string; url: string } | string;
  resolutions?: Record<string, string>;
  scripts?: Record<string, string>;
  version?: string;
  workspaces?: Record<string, string[]> | string[];
  [otherProps: string]:
    | Record<string, string | string[]>
    | string
    | string[]
    | undefined;
}

export interface SourceWrapper {
  /** the absolute path on disk to this package.json file */
  readonly filePath: string;
  /** the parsed JSON contents of this package.json file */
  contents: Source;
  /** the raw file contents of this package.json file */
  readonly json: string;
}

/**
 * Read the file contents and metadata for every package.json file needed.
 */
export function getWrappers(
  disk: Disk,
  program: SyncpackConfig,
): SourceWrapper[] {
  const useEmpty = () => [];
  return pipe(
    getFilePaths(disk, program),
    E.chain(
      flow(
        O.getOrElse(useEmpty as () => string[]),
        E.traverseArray(readJsonSafe(disk)),
        E.map(removeReadonlyType),
      ),
    ),
    E.fold(useEmpty as () => SourceWrapper[], (wrappers) => wrappers),
  );
}
