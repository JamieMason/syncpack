import chalk from 'chalk';
import { DEFAULT_CONFIG } from './constants';

export const option = {
  config: ['-c, --config <path>', 'path to a syncpack config file'],
  dev: ['-d, --dev', chalk`include {yellow devDependencies}`],
  filter: [
    '-f, --filter [pattern]',
    chalk`only include dependencies whose {yellow name} matches this regex`,
  ],
  indent: [
    '-i, --indent [value]',
    `override indentation. defaults to "${DEFAULT_CONFIG.indent}"`,
  ],
  overrides: ['-o, --overrides', chalk`include {yellow overrides} (npm)`],
  peer: ['-P, --peer', chalk`include {yellow peerDependencies}`],
  pnpmOverrides: [
    '-O, --pnpmOverrides',
    chalk`include {yellow overrides} (pnpm)`,
  ],
  prod: ['-p, --prod', chalk`include {yellow dependencies}`],
  resolutions: [
    '-R, --resolutions',
    chalk`include {yellow resolutions} (yarn)`,
  ],
  semverRange: [
    '-r, --semver-range <range>',
    `see supported ranges below. defaults to "${DEFAULT_CONFIG.semverRange}"`,
  ],
  source: [
    '-s, --source [pattern]',
    'glob pattern for package.json files to read from',
    collect,
    [] as string[],
  ],
  workspace: ['-w, --workspace', 'include locally developed package versions'],
} as const;

function collect(value: string, previous: string[]): string[] {
  return previous.concat([value]);
}
