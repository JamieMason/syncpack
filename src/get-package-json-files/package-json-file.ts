import { Effect, pipe } from 'effect';
import type { Strategy } from '../config/get-custom-types.js';
import type { Ctx } from '../get-context/index.js';
import { Instance } from '../get-instances/instance.js';
import type { RcFile } from '../index.js';
import type { JsonFile } from '../io/read-json-file-sync.js';

export type PackageJson = {
  bugs?: { url: string } | string;
  config?: { syncpack?: RcFile };
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
} & Record<
  string,
  | Record<string, string | string[] | Record<string, string | string[]>>
  | string
  | string[]
  | undefined
>;

export class PackageJsonFile {
  /** resolved configuration */
  readonly config: Ctx['config'];
  /** ensure only one set of instances is ever created and shared */
  private _instances: Instance[] | null;

  /** the wrapped package.json file */
  jsonFile: JsonFile<PackageJson>;

  /** the .name property from the package.json file */
  name: string | undefined;

  constructor(jsonFile: JsonFile<PackageJson>, config: Ctx['config']) {
    this._instances = null;
    this.config = config;
    this.jsonFile = jsonFile;
    this.name = jsonFile.contents.name;
  }

  getInstances(enabledTypes: Strategy.Any[]): Effect.Effect<never, never, Instance[]> {
    if (!this._instances) {
      return pipe(
        Effect.all(
          enabledTypes.map((strategy) =>
            pipe(
              strategy.read(this),
              Effect.map((entries) =>
                entries.map(
                  ([name, rawSpecifier]) => new Instance(name, rawSpecifier, this, strategy),
                ),
              ),
            ),
          ),
        ),
        Effect.map((array) => array.flat()),
        Effect.tapBoth({
          onSuccess: (instances) =>
            Effect.logDebug(`found ${instances.length} instances in <${this.jsonFile.shortPath}>`),
          onFailure: () =>
            Effect.logError(`failed to get instances from <${this.jsonFile.shortPath}>`),
        }),
        Effect.catchAll(() => Effect.succeed([])),
        Effect.tap((instances) =>
          Effect.sync(() => {
            this._instances = instances;
          }),
        ),
      );
    }
    return Effect.succeed(this._instances);
  }
}
