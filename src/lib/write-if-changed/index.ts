import chalk from 'chalk';
import { relative } from 'path';
import { CWD, ICON } from '../../constants';
import type { Disk } from '../disk';
import type { Source } from '../get-input/get-wrappers';
import { detectNewlines, setNewlines } from './set-newlines';

interface FileData {
  contents: Source;
  filePath: string;
  indent: string;
  json: string;
}

export function writeIfChanged(disk: Disk, fileData: FileData): void {
  const { contents, filePath, indent, json } = fileData;
  const EOL = detectNewlines(json);
  const shortPath = relative(CWD, filePath);
  const source = `${JSON.stringify(contents, null, indent)}${EOL}`;
  const after = setNewlines(source, EOL);
  if (json !== after) {
    disk.writeFileSync(filePath, after);
    console.log(chalk.green(ICON.tick), shortPath);
  } else {
    console.log(chalk.dim(ICON.skip), chalk.dim(shortPath));
  }
}
