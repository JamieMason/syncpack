#!/usr/bin/env node

import chalk from 'chalk';
import { program } from 'commander';
import { effects } from '../lib/effects';
import { showHelpOnError } from '../lib/show-help-on-error';
import { option } from '../option';
import { listCli } from './list-cli';

program.description('  List all dependencies required by your packages.');

program.on('--help', () => {
  console.log(chalk`
Examples:
  {dim # uses defaults for resolving packages}
  syncpack list
  {dim # uses packages defined by --source when provided}
  syncpack list --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack list --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # uses dependencies regular expression defined by --filter when provided}
  syncpack list --filter {yellow "typescript|tslint"}
  {dim # only inspect "devDependencies"}
  syncpack list --types dev
  {dim # only inspect "devDependencies" and "peerDependencies"}
  syncpack list --types dev,peer

Resolving Packages:
  1. If {yellow --source} globs are provided, use those.
  2. If using Pnpm Workspaces, read {yellow packages} from {yellow pnpm-workspace.yaml} in the root of the project.
  3. If using Yarn Workspaces, read {yellow workspaces} from {yellow package.json}.
  4. If using Lerna, read {yellow packages} from {yellow lerna.json}.
  5. Default to {yellow "package.json"} and {yellow "packages/*/package.json"}.

Reference:
  globs            {blue.underline https://github.com/isaacs/node-glob#glob-primer}
  lerna.json       {blue.underline https://github.com/lerna/lerna#lernajson}
  Yarn Workspaces  {blue.underline https://yarnpkg.com/lang/en/docs/workspaces}
  Pnpm Workspaces  {blue.underline https://pnpm.js.org/en/workspaces}
`);
});

showHelpOnError(program);

program
  .option(...option.source)
  .option(...option.filter)
  .option(...option.config)
  .option(...option.types)
  .parse(process.argv);

listCli(
  {
    configPath: program.opts().config,
    filter: program.opts().filter,
    source: program.opts().source,
    types: program.opts().types,
  },
  effects,
);
