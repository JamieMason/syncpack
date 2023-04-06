import { relative } from 'path';
import { pipe } from 'tightrope/fn/pipe';
import { Ok } from 'tightrope/result';
import { unwrapOr } from 'tightrope/result/unwrap-or';
import { CWD } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { verbose } from '../../../lib/log';
import { newlines } from '../../../lib/newlines';
import type { Syncpack } from '../../../types';
import {
  exhaustiveCheck,
  strategyByName
} from '../../get-config/path-strategy';
import type { JsonFile } from '../get-patterns/read-json-safe';
import { Instance } from './instance';

export interface PackageJson {
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

type Entry = [string, string];

export class PackageJsonFile {
  /** parsed JSON contents of the file */
  contents: PackageJson;

  /** api for writing to disk */
  readonly disk: Disk;

  /** absolute path on disk to this file */
  readonly filePath: string;

  /** raw file contents of the file */
  readonly json: string;

  /** resolved configuration */
  readonly config: Syncpack.Config.Private;

  /** relative path on disk to this file */
  readonly shortPath: string;

  constructor(
    jsonFile: JsonFile<PackageJson>,
    config: Syncpack.Config.Private,
    disk: Disk,
  ) {
    this.contents = jsonFile.contents;
    this.disk = disk;
    this.filePath = jsonFile.filePath;
    this.json = jsonFile.json;
    this.config = config;
    this.shortPath = relative(CWD, jsonFile.filePath);
  }

  hasChanged(): boolean {
    return this.json !== this.getSource();
  }

  write(): void {
    this.disk.writeFileSync(this.filePath, this.getSource());
  }

  getSource(): string {
    const contents = this.contents;
    const indent = this.config.indent;
    const EOL = newlines.detect(this.json);
    const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
    return newlines.fix(source, EOL);
  }

  getInstances(): Instance[] {
    return this.config.enabledTypes
      .flatMap((pathDef): Instance[] =>
        this.getPathEntries(pathDef, this).map(
          ([name, version]) => new Instance(pathDef, name, this, version),
        ),
      )
      .filter((instance) => {
        const { pathDef, name, version } = instance;
        const filter = this.config.filter;
        if (name.search(new RegExp(filter)) === -1) {
          verbose(`skip, name "${name}" does not match filter "${filter}"`);
          return false;
        }
        verbose(
          `add ${name}@${version} to ${pathDef.name}:${pathDef.strategy} ${this.shortPath}`,
        );
        return true;
      });
  }

  getPathEntries(
    pathDef: Syncpack.PathDefinition,
    file: PackageJsonFile,
  ): Entry[] {
    const strategyName = pathDef.strategy;
    switch (strategyName) {
      case 'name@version':
        return pipe(
          strategyByName[strategyName].read(file, pathDef),
          unwrapOr(new Ok([] as Entry[])),
        );
      case 'name~version':
        return pipe(
          strategyByName[strategyName].read(file, pathDef),
          unwrapOr(new Ok([] as Entry[])),
        );
      case 'version':
        return pipe(
          strategyByName[strategyName].read(file, pathDef),
          unwrapOr(new Ok([] as Entry[])),
        );
      case 'versionsByName':
        return pipe(
          strategyByName[strategyName].read(file, pathDef),
          unwrapOr(new Ok([] as Entry[])),
        );
      default:
        return exhaustiveCheck(strategyName);
    }
  }
}
