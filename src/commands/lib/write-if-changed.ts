import chalk from 'chalk';
import { writeFileSync } from 'fs-extra';
import { EOL } from 'os';
import { relative } from 'path';
import { SourceWrapper } from './get-wrappers';
import { log } from './log';

export const writeIfChanged = (indent: string, wrapper: SourceWrapper, mutateContents: () => void): void => {
  const toJson = (): string => `${JSON.stringify(wrapper.contents, null, indent)}${EOL}`;
  const shortPath = relative(process.cwd(), wrapper.filePath);
  const before = toJson();
  mutateContents();
  const after = toJson();
  if (before !== after) {
    writeFileSync(wrapper.filePath, after);
    log(chalk.green('✓'), shortPath);
  } else {
    log(chalk.dim('-'), chalk.dim(shortPath));
  }
};
