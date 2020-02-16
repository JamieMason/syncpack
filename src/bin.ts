#!/usr/bin/env node

import program = require('commander');

program
  .version(require('../package.json').version)
  .command('fix-mismatches', 'set dependencies used with different versions to the same version')
  .command('format', 'sort and shorten properties according to a convention')
  .command('list', 'list every dependency used in your packages', { isDefault: true })
  .command('list-mismatches', 'list every dependency used with different versions in your packages')
  .command('set-semver-ranges', 'set semver ranges to the given format')
  .parse(process.argv);
