#!/usr/bin/env node

import { program } from 'commander';

program
  .version(require('../package.json').version)
  .command('fix-mismatches', 'set dependencies used with different versions to the same version')
  .command('format', 'sort and shorten properties according to a convention')
  .command('lint-semver-ranges', 'check dependency versions comply with the given semver range format')
  .command('list-mismatches', 'list every dependency used with different versions in your packages')
  .command('list', 'list every dependency used in your packages', { isDefault: true })
  .command('set-semver-ranges', 'set semver ranges to the given format')
  .parse(process.argv);
