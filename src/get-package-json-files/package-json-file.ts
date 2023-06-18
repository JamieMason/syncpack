import { pipe } from '@effect/data/Function';
import { relative } from 'path';
import { map } from 'tightrope/result/map';
import type { Strategy } from '../config/get-custom-types';
import { CWD } from '../constants';
import type { Ctx } from '../get-context';
import { logVerbose } from '../lib/log-verbose';
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
  workspaces?: string[] | { packages?: string[] };
  [otherProps: string]:
    | Record<string, string | string[] | Record<string, string | string[]>>
    | string
    | string[]
    | undefined;
}

export class PackageJsonFile {
  /** parsed JSON contents of the file */
  contents: PackageJson;

  /** absolute path on disk to this file */
  readonly filePath: string;

  /** raw file contents of the file */
  readonly json: string;

  /** resolved configuration */
  readonly config: Ctx['config'];

  /** relative path on disk to this file */
  readonly shortPath: string;

  constructor(jsonFile: JsonFile<PackageJson>, config: Ctx['config']) {
    this.config = config;
    this.contents = jsonFile.contents;
    this.filePath = jsonFile.filePath;
    this.json = jsonFile.json;
    this.shortPath = relative(CWD, jsonFile.filePath);
  }

  getInstances(enabledTypes: Strategy.Any[]): Instance[] {
    const instances: Instance[] = [];

    enabledTypes.forEach((strategy) => {
      pipe(
        strategy.read(this),
        map((entries) =>
          entries.forEach(([name, version]) => {
            logVerbose(
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
