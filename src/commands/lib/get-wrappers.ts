import { isArrayOfStrings } from 'expect-more';
import { readJsonSync } from 'fs-extra';
import { sync } from 'glob';
import { join, resolve } from 'path';
import { sync as readYamlFileSync } from 'read-yaml-file';
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
  /** the absolute path on disk to this package.json file */
  filePath: string;
  /** the parsed JSON contents of this package.json file */
  contents: Source;
}

const getPatternsFromJson = (
  fileName: string,
  getProperties: (config: any) => Array<string | undefined>,
): string[] | null => {
  const filePath = resolve(process.cwd(), fileName);
  const config = readJsonSync(filePath, { throws: false });
  if (!config) return null;
  const packages = getProperties(config).find(isArrayOfStrings);
  return packages ? [process.cwd()].concat(packages).map((dirPath) => join(dirPath, 'package.json')) : null;
};

const getCliPatterns = (program: Options): Options['source'] | null =>
  isArrayOfStrings(program.source) ? program.source : null;

const getYarnPatterns = (): string[] | null =>
  getPatternsFromJson('package.json', (config) => [config.workspaces, config.workspaces?.packages]);

const getLernaPatterns = (): string[] | null => getPatternsFromJson('lerna.json', (config) => [config.packages]);

const getPnpmPatterns = (): string[] | null => {
  try {
    const filePath = resolve(process.cwd(), 'pnpm-workspace.yaml');
    const config = readYamlFileSync<{ packages?: string[] }>(filePath);
    const packages = [config.packages].find(isArrayOfStrings);
    return packages ? [process.cwd()].concat(packages).map((dirPath) => join(dirPath, 'package.json')) : null;
  } catch (err) {
    return null;
  }
};

const getDefaultPatterns = (): string[] => ALL_PATTERNS;
const resolvePattern = (pattern: string): string[] => sync(pattern, { absolute: true });
const reduceFlatArray = (all: string[], next: string[]): string[] => all.concat(next);
const createWrapper = (filePath: string): SourceWrapper => ({ contents: readJsonSync(filePath), filePath });

export const getWrappers = (program: Options): SourceWrapper[] =>
  (getCliPatterns(program) || getYarnPatterns() || getPnpmPatterns() || getLernaPatterns() || getDefaultPatterns())
    .map(resolvePattern)
    .reduce(reduceFlatArray, [])
    .map(createWrapper);
