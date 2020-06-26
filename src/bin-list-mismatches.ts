#!/usr/bin/env node

import chalk from 'chalk';
import program = require('commander');
import { listMismatchesFromDisk } from './commands/list-mismatches';
import { option } from './constants';
import { parseFilterArgs } from './lib/parse-filter-args';

program.description(
  `
  List dependencies which are required by multiple packages, where the version
  is not the same across every package.`.replace(/^\n/, ''),
);

program.on('--help', () => {
  console.log(chalk`
Examples:
  {dim # uses defaults for resolving packages}
  syncpack list-mismatches
  {dim # uses packages defined by --source when provided}
  syncpack list-mismatches --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack list-mismatches --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # uses dependencies regular expression defined by --filter when provided}
  syncpack list-mismatches --filter {yellow "typescript|tslint"}
  {dim # multiple filters can be provided like this}
  syncpack list-mismatches --filter {yellow "@react"} --filter {yellow "webpack"}
  {dim # only inspect "devDependencies"}
  syncpack list-mismatches --dev
  {dim # only inspect "devDependencies" and "peerDependencies"}
  syncpack list-mismatches --dev --peer

Resolving Packages:
  1. If {yellow --source} globs are provided, use those.
  2. If using Yarn Workspaces, read {yellow workspaces} from {yellow package.json}.
  3. If using Lerna, read {yellow packages} from {yellow lerna.json}.
  4. Default to {yellow "package.json"} and {yellow "packages/*/package.json"}.

Reference:
  globs            {blue.underline https://github.com/isaacs/node-glob#glob-primer}
  lerna.json       {blue.underline https://github.com/lerna/lerna#lernajson}
  Yarn Workspaces  {blue.underline https://yarnpkg.com/lang/en/docs/workspaces}
`);
});

program
  .option(...option.source)
  .option(...option.prod)
  .option(...option.dev)
  .option(...option.peer)
  .option(...option.filter)
  .parse(process.argv);

listMismatchesFromDisk({
  dev: Boolean(program.dev),
  filter: parseFilterArgs(program.filter),
  peer: Boolean(program.peer),
  prod: Boolean(program.prod),
  sources: Array.isArray(program.source) ? program.source : [],
});
