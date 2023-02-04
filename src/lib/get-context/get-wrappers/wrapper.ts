import type { Source } from '.';
import type { Disk } from '../../disk';
import type { InternalConfig } from '../get-config/internal-config';
import type { JsonFile } from './get-patterns/read-json-safe';
import { newlines } from './newlines';

export class Wrapper {
  /** parsed JSON contents of the file */
  contents: Source;

  /** api for writing to disk */
  readonly disk: Disk;

  /** absolute path on disk to this file */
  readonly filePath: string;

  /** raw file contents of the file */
  readonly json: string;

  /** resolved configuration */
  readonly program: Pick<InternalConfig, 'indent'>;

  constructor(
    jsonFile: JsonFile<Source>,
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
