#!/usr/bin/env node

import chalk from 'chalk';
import { program } from 'commander';
import { disk } from '../lib/disk';
import { getInput } from '../lib/get-input';
import { option } from '../option';
import { fixMismatches } from './fix-mismatches';

program.description(
  chalk`
  Ensure that multiple packages requiring the same dependency define the same
  version, so that every package requires eg. {yellow react@16.4.2}, instead of a
  combination of {yellow react@16.4.2}, {yellow react@0.15.9}, and {yellow react@16.0.0}.`.replace(
    /^\n/,
    '',
  ),
);

program.on('--help', () => {
  console.log(chalk`
Resolving Packages:
  1. If {yellow --source} globs are provided, use those.
  2. If using Pnpm Workspaces, read {yellow packages} from {yellow pnpm-workspace.yaml} in the root of the project.
  3. If using Yarn Workspaces, read {yellow workspaces} from {yellow package.json}.
  4. If using Lerna, read {yellow packages} from {yellow lerna.json}.
  5. Default to {yellow "package.json"} and {yellow "packages/*/package.json"}.

Examples:
  {dim # uses defaults for resolving packages}
  syncpack fix-mismatches
  {dim # uses packages defined by --source when provided}
  syncpack fix-mismatches --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack fix-mismatches --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # uses dependencies regular expression defined by --filter when provided}
  syncpack fix-mismatches --filter {yellow "typescript|tslint"}
  {dim # only inspect "devDependencies"}
  syncpack fix-mismatches --dev
  {dim # only inspect "devDependencies" and "peerDependencies"}
  syncpack fix-mismatches --dev --peer
  {dim # indent package.json with 4 spaces instead of 2}
  syncpack fix-mismatches --indent {yellow "    "}

Reference:
  globs            {blue.underline https://github.com/isaacs/node-glob#glob-primer}
  lerna.json       {blue.underline https://github.com/lerna/lerna#lernajson}
  Yarn Workspaces  {blue.underline https://yarnpkg.com/lang/en/docs/workspaces}
  Pnpm Workspaces  {blue.underline https://pnpm.js.org/en/workspaces}
`);
});

program
  .option(...option.source)
  .option(...option.filter)
  .option(...option.config)
  .option(...option.prod)
  .option(...option.dev)
  .option(...option.peer)
  .option(...option.resolutions)
  .option(...option.overrides)
  .option(...option.workspace)
  .option(...option.indent)
  .parse(process.argv);

fixMismatches(
  getInput(disk, {
    configPath: program.opts().config,
    dev: program.opts().dev,
    filter: program.opts().filter,
    indent: program.opts().indent,
    overrides: program.opts().overrides,
    peer: program.opts().peer,
    pnpmOverrides: program.opts().pnpmOverrides,
    prod: program.opts().prod,
    resolutions: program.opts().resolutions,
    source: program.opts().source,
    workspace: program.opts().workspace,
  }),
  disk,
);
