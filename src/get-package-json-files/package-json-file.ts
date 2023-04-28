import { relative } from 'path';
import { pipe } from 'tightrope/fn/pipe';
import { map } from 'tightrope/result/map';
import type { Strategy } from '../config/get-custom-types';
import { getIndent } from '../config/get-indent';
import { CWD } from '../constants';
import type { Context } from '../get-context';
import type { Disk } from '../lib/disk';
import { verbose } from '../lib/log';
import { newlines } from '../lib/newlines';
import type { JsonFile } from './get-patterns/read-json-safe';
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
  readonly config: Context['config'];

  /** relative path on disk to this file */
  readonly shortPath: string;

  constructor(
    jsonFile: JsonFile<PackageJson>,
    config: Context['config'],
    disk: Disk,
  ) {
    this.config = config;
    this.contents = jsonFile.contents;
    this.disk = disk;
    this.filePath = jsonFile.filePath;
    this.json = jsonFile.json;
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
    const indent = getIndent(this.config);
    const EOL = newlines.detect(this.json);
    const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
    return newlines.fix(source, EOL);
  }

  getInstances(enabledTypes: Strategy.Any[]): Instance[] {
    const instances: Instance[] = [];

    enabledTypes.forEach((strategy) => {
      pipe(
        strategy.read(this),
        map((entries) =>
          entries.forEach(([name, version]) => {
            verbose(
              `add ${name}@${version} to ${strategy.name}:${strategy._tag} ${this.shortPath}`,
            );
            instances.push(new Instance(strategy, name, this, version));
          }),
        ),
      );
    });

    return instances;
  }
}
