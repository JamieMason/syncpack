import { pipe } from '@effect/data/Function';
import { map } from 'tightrope/result/map';
import type { Strategy } from '../config/get-custom-types';
import type { JsonFile } from '../env/tags';
import type { Ctx } from '../get-context';
import type { Instance } from '../instance';
import { createInstance } from '../instance/create';
import { logVerbose } from '../lib/log-verbose';

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
  /** resolved configuration */
  readonly config: Ctx['config'];

  /** the wrapped package.json file */
  jsonFile: JsonFile<PackageJson>;

  constructor(jsonFile: JsonFile<PackageJson>, config: Ctx['config']) {
    this.config = config;
    this.jsonFile = jsonFile;
  }

  getInstances(enabledTypes: Strategy.Any[]): Instance.Any[] {
    const instances: Instance.Any[] = [];

    enabledTypes.forEach((strategy) => {
      pipe(
        strategy.read(this),
        map((entries) =>
          entries.forEach(([name, specifier]) => {
            logVerbose(
              `add ${name}@${specifier} to ${strategy.name}:${strategy._tag} ${this.jsonFile.shortPath}`,
            );
            instances.push(createInstance(strategy, name, this, specifier));
          }),
        ),
      );
    });

    return instances;
  }
}
