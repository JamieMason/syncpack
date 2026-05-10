#!/usr/bin/env node

const { spawnSync } = require('node:child_process');
const { dirname, join } = require('node:path');

const args = process.argv.slice(2);
const arch = process.arch;
const [os, extension] = ['win32', 'cygwin'].includes(process.platform) ? ['windows', '.exe'] : [process.platform, ''];
const libc = isMusl() ? '-musl' : '';
const optionalDep = `syncpack-${os}-${arch}${libc}`;
const binaryName = `syncpack${extension}`;

const pathToBinary = resolveBinaryPath();

process.exit(
  spawnSync(pathToBinary, args, {
    cwd: process.cwd(),
    stdio: 'inherit',
    env: process.env,
  }).status || 0,
);

function isMusl() {
  try {
    if (process.platform !== 'linux') return false;
    const { sharedObjects } = process.report.getReport();
    return sharedObjects.some(obj => obj.includes('musl'));
  } catch (_) {
    return false;
  }
}

function resolveBinaryPath() {
  // Strategy 1: Resolve via package.json for pnpm Plug'n'Play
  try {
    const packageJsonPath = require.resolve(`${optionalDep}/package.json`);
    const packageDir = dirname(packageJsonPath);
    return join(packageDir, 'bin', binaryName);
  } catch (_) {}

  // Strategy 2: Original approach (works with traditional node_modules)
  try {
    return require.resolve(`${optionalDep}/bin/${binaryName}`);
  } catch (_) {}

  throw new Error(
    `Failed to resolve binary for ${os}-${arch}${libc}. Please ensure ${optionalDep} is installed as an optional dependency.`,
  );
}
