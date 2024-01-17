#!/usr/bin/env node

import { program } from 'commander';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

program
  .version(JSON.parse(fs.readFileSync(`${__dirname}/../package.json`, 'utf8')).version)
  .command('fix-mismatches', 'set dependencies used with different versions to the same version', {
    executableFile: './bin-fix-mismatches/index.js',
  })
  .command('format', 'sort and shorten properties according to a convention', {
    executableFile: './bin-format/index.js',
  })
  .command('lint', 'lint all versions and ranges', {
    executableFile: './bin-lint/index.js',
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
  })
  .command('prompt', 'fix mismatches which syncpack cannot fix automatically', {
    executableFile: './bin-prompt/index.js',
  })
  .command('set-semver-ranges', 'set semver ranges to the given format', {
    executableFile: './bin-set-semver-ranges/index.js',
  })
  .command('update', 'update to the latest versions on the npm registry', {
    executableFile: './bin-update/index.js',
  })
  .parse(process.argv);
