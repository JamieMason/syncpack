import chalk from 'chalk';
import { DEFAULT_CONFIG } from './constants';
import { collect } from './lib/collect';

export const option = {
  dev: ['-d, --dev', chalk`include {yellow devDependencies}`],
  filter: [
    '-f, --filter [pattern]',
    chalk`only include dependencies whose {yellow name} matches this regex`,
  ],
  indent: [
    '-i, --indent [value]',
    `override indentation. defaults to "${DEFAULT_CONFIG.indent}"`,
  ],
  overrides: ['-o, --overrides', chalk`include {yellow overrides} (pnpm)`],
  peer: ['-P, --peer', chalk`include {yellow peerDependencies}`],
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
} as const;
