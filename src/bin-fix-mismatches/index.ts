#!/usr/bin/env node

import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { program } from 'commander';
import { defaultEnv } from '../env/default-env';
import { showHelpOnError } from '../lib/show-help-on-error';
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
  syncpack fix-mismatches --types dev
  {dim # only inspect "devDependencies" and "peerDependencies"}
  syncpack fix-mismatches --types dev,peer
  {dim # indent package.json with 4 spaces instead of 2}
  syncpack fix-mismatches --indent {yellow "    "}

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
  .option(...option.types)
  .option(...option.config)
  .option(...option.indent)
  .parse(process.argv);

Effect.runSync<never, unknown>(
  fixMismatches(
    {
      configPath: program.opts().config,
      filter: program.opts().filter,
      indent: program.opts().indent,
      source: program.opts().source,
      types: program.opts().types,
    },
    defaultEnv,
  ),
);
