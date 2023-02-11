import { isNonEmptyString } from 'expect-more';
import { relative } from 'path';
import { CWD } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { verbose } from '../../../lib/log';
import { newlines } from '../../../lib/newlines';
import type { Syncpack } from '../../../types';
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
  readonly program: Syncpack.Config.Private;

  /** relative path on disk to this file */
  readonly shortPath: string;

  constructor(
    jsonFile: JsonFile<PackageJson>,
    program: Syncpack.Config.Private,
    disk: Disk,
  ) {
    this.contents = jsonFile.contents;
    this.disk = disk;
    this.filePath = jsonFile.filePath;
    this.json = jsonFile.json;
    this.program = program;
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
    const indent = this.program.indent;
    const EOL = newlines.detect(this.json);
    const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
    return newlines.fix(source, EOL);
  }

  getInstances(): Instance[] {
    return this.program.dependencyTypes
      .flatMap((dependencyType): Instance[] =>
        this.getDependencyEntries(dependencyType, this.contents).map(
          ([name, version]) =>
            new Instance(dependencyType, name, this, version),
        ),
      )
      .filter((instance) => {
        const { dependencyType, name, version } = instance;
        if (!isNonEmptyString(name)) {
          verbose('skip instance, no name', instance);
          return false;
        }
        if (name.search(new RegExp(this.program.filter)) === -1) {
          verbose('skip instance, name does not match filter', instance);
          return false;
        }
        if (!isNonEmptyString(version)) {
          verbose('skip instance, no version', instance);
          return false;
        }
        verbose(`add ${name}@${version} to ${dependencyType} ${this.filePath}`);
        return true;
      });
  }

  getDependencyEntries(
    dependencyType: Syncpack.Config.DependencyType.Name,
    contents: PackageJson,
  ): [string, string][] {
    switch (dependencyType) {
      case 'dependencies':
      case 'devDependencies':
      case 'overrides':
      case 'peerDependencies':
      case 'resolutions': {
        return Object.entries(contents?.[dependencyType] || {});
      }
      case 'workspace': {
        return [[contents.name || '', contents.version || '']];
      }
      case 'pnpmOverrides': {
        return Object.entries(contents?.pnpm?.overrides || {});
      }
    }
  }
}
