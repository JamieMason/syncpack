#!/usr/bin/env node

const { spawnSync } = require('node:child_process');

const args = process.argv.slice(2);
const arch = process.arch;
const [os, extension] = ['win32', 'cygwin'].includes(process.platform)
  ? ['windows', '.exe']
  : [process.platform, ''];
const optionalDep = `syncpack-${os}-${arch}`;
const pkgSpecifier = `${optionalDep}/bin/syncpack${extension}`;
const pathToBinary = require.resolve(pkgSpecifier);

process.exit(
  spawnSync(pathToBinary, args, {
    cwd: process.cwd(),
    stdio: ['ignore', 'inherit', 'inherit'],
    env: {
      ...process.env,
      COSMICONFIG_REQUIRE_PATH: require.resolve('cosmiconfig'),
      RUST_BACKTRACE: 'full',
    },
  }).status || 0,
);
