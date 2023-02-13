import chalk from 'chalk';
import { DEFAULT_CONFIG } from './constants';

export const option = {
  config: ['-c, --config <path>', 'path to a syncpack config file'],
  filter: [
    '-f, --filter [pattern]',
    chalk`only include dependencies whose {yellow name} matches this regex`,
  ],
  indent: [
    '-i, --indent [value]',
    `override indentation. defaults to "${DEFAULT_CONFIG.indent}"`,
  ],
  semverRange: [
    '-r, --semver-range <range>',
    `see supported ranges below. defaults to "${DEFAULT_CONFIG.semverRange}"`,
  ],
  source: [
    '-s, --source [pattern]',
    'glob pattern for package.json files to read from',
    collect,
  ],
  types: [
    '-t, --types <names>',
    chalk`only include dependencies matching these types (eg. {yellow types=dev,prod,myCustomType})`,
  ],
} as const;

function collect(value: string, previous: string[]): string[] {
  return previous.concat([value]);
}
