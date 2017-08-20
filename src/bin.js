#!/usr/bin/env node

import 'nodent-runtime';
import program from 'commander';
import { version } from '../package.json';
import { DEFAULT_PACKAGES, DEFAULT_SOURCE } from './constants';

program
  .version(version)
  .command('sync-versions', 'synchronise dependency versions between packages', {
    isDefault: true
  })
  .command('copy-values <keys...>', `copy values from eg. ${DEFAULT_SOURCE} to ${DEFAULT_PACKAGES}`);

program.parse(process.argv);
