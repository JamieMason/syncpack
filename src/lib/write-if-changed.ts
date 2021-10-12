import chalk from 'chalk';
import { EOL } from 'os';
import { relative } from 'path';
import { CWD } from '../constants';
import type { Source } from '../lib/get-input/get-wrappers';
import type { Disk } from './disk';

interface FileData {
  contents: Source;
  filePath: string;
  indent: string;
  json: string;
}

export function writeIfChanged(disk: Disk, fileData: FileData): void {
  const { contents, filePath, indent, json } = fileData;
  const shortPath = relative(CWD, filePath);
  const after = `${JSON.stringify(contents, null, indent)}${EOL}`;
  if (json !== after) {
    disk.writeFileSync(filePath, after);
    console.log(chalk.green('✓'), shortPath);
  } else {
    console.log(chalk.dim('-'), chalk.dim(shortPath));
  }
}
