import type { Disk } from '../../disk';
import { newlines } from '../../newlines';
import type { InternalConfig } from '../get-config/internal-config';
import type { JsonFile } from './get-patterns/read-json-safe';

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
  readonly program: Pick<InternalConfig, 'indent'>;

  constructor(
    jsonFile: JsonFile<PackageJson>,
    program: Pick<InternalConfig, 'indent'>,
    disk: Disk,
  ) {
    this.contents = jsonFile.contents;
    this.disk = disk;
    this.filePath = jsonFile.filePath;
    this.json = jsonFile.json;
    this.program = program;
  }

  getSource(): string {
    const contents = this.contents;
    const indent = this.program.indent;
    const EOL = newlines.detect(this.json);
    const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
    return newlines.fix(source, EOL);
  }

  hasChanged(): boolean {
    return this.json !== this.getSource();
  }

  write(): void {
    this.disk.writeFileSync(this.filePath, this.getSource());
  }
}
