import chalk from 'chalk';
import { writeFileSync } from 'fs-extra';
import { EOL } from 'os';
import { relative } from 'path';
import { CWD } from '../../constants';
import { SourceWrapper } from './get-wrappers';
import { log } from './log';

const toJson = (indent: string, wrapper: SourceWrapper): string =>
  `${JSON.stringify(wrapper.contents, null, indent)}${EOL}`;

export const writeIfChanged = (indent: string, wrapper: SourceWrapper, mutateContents: () => void): void => {
  const shortPath = relative(CWD, wrapper.filePath);
  mutateContents();
  const after = toJson(indent, wrapper);
  if (wrapper.json !== after) {
    writeFileSync(wrapper.filePath, after);
    log(chalk.green('✓'), shortPath);
  } else {
    log(chalk.dim('-'), chalk.dim(shortPath));
  }
};
