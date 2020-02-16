import { readJsonSync } from 'fs-extra';
import { sync } from 'glob';
import { join, resolve } from 'path';
import { ALL_PATTERNS } from '../../constants';

interface Options {
  source: string[];
}

export interface Source {
  bugs?: { url: string } | string;
  dependencies?: { [key: string]: string };
  description?: string;
  devDependencies?: { [key: string]: string };
  keywords?: string[];
  name?: string;
  peerDependencies?: { [key: string]: string };
  repository?: { type: string; url: string } | string;
  resolutions?: { [key: string]: string };
  scripts?: { [key: string]: string };
  version?: string;
  [otherProps: string]: string | string[] | { [key: string]: string } | undefined;
}

export interface SourceWrapper {
  filePath: string;
  contents: Source;
}

const getPatternsFromConfig = (fileName: string, propName: string): string[] | null => {
  const filePath = resolve(process.cwd(), fileName);
  const config = readJsonSync(filePath, { throws: false });
  const isNonEmptyArray = config && config[propName] && config[propName].length > 0;
  return isNonEmptyArray
    ? [process.cwd()].concat(config[propName]).map((dirPath: string) => join(dirPath, 'package.json'))
    : null;
};

const hasCliPatterns = (program: Options) => program.source && program.source.length > 0;
const getCliPatterns = (program: Options) => program.source;
const getYarnPatterns = () => getPatternsFromConfig('package.json', 'workspaces');
const getLernaPatterns = () => getPatternsFromConfig('lerna.json', 'packages');
const getDefaultPatterns = () => ALL_PATTERNS;
const resolvePattern = (pattern: string) => sync(pattern, { absolute: true });
const reduceFlatArray = (all: string[], next: string[]) => all.concat(next);
const createWrapper = (filePath: string) => ({ contents: readJsonSync(filePath), filePath });

export const getWrappers = (program: Options): SourceWrapper[] =>
  (hasCliPatterns(program) ? getCliPatterns(program) : getYarnPatterns() || getLernaPatterns() || getDefaultPatterns())
    .map(resolvePattern)
    .reduce(reduceFlatArray, [])
    .map(createWrapper);
