#!/usr/bin/env node

import { program } from 'commander';

program
  .version(require('../package.json').version)
  .command(
    'fix-mismatches',
    'set dependencies used with different versions to the same version',
    {
      executableFile: './bin-fix-mismatches/index.js',
    },
  )
  .command('format', 'sort and shorten properties according to a convention', {
    executableFile: './bin-format/index.js',
  })
  .command(
    'lint-semver-ranges',
    'check dependency versions comply with the given semver range format',
    {
      executableFile: './bin-lint-semver-ranges/index.js',
    },
  )
  .command(
    'list-mismatches',
    'list every dependency used with different versions in your packages',
    {
      executableFile: './bin-list-mismatches/index.js',
    },
  )
  .command('list', 'list every dependency used in your packages', {
    executableFile: './bin-list/index.js',
    isDefault: true,
  })
  .command('set-semver-ranges', 'set semver ranges to the given format', {
    executableFile: './bin-set-semver-ranges/index.js',
  })
  .parse(process.argv);
