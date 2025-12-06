#!/usr/bin/env node

const { spawnSync } = require('node:child_process');
const { dirname, join } = require('node:path');

const args = process.argv.slice(2);
const arch = process.arch;
const [os, extension] = ['win32', 'cygwin'].includes(process.platform) ? ['windows', '.exe'] : [process.platform, ''];
const optionalDep = `syncpack-${os}-${arch}`;
const binaryName = `syncpack${extension}`;

const pathToBinary = resolveBinaryPath();

process.exit(
  spawnSync(pathToBinary, args, {
    cwd: process.cwd(),
    stdio: ['ignore', 'inherit', 'inherit'],
    env: process.env,
  }).status || 0,
);

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
    `Failed to resolve binary for ${os}-${arch}. Please ensure ${optionalDep} is installed as an optional dependency.`,
  );
}
