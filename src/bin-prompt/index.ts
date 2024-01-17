#!/usr/bin/env node

import chalk from 'chalk-template';
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { prompt } from './prompt.js';

program.description(
  '  displays a series of prompts to fix mismatches which syncpack cannot fix automatically',
);

program.on('--help', () => {
  console.log(chalk`
  Examples:
  {dim # uses defaults for resolving packages}
  syncpack prompt
  {dim # uses packages defined by --source when provided}
  syncpack prompt --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack prompt --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # uses dependencies regular expression defined by --filter when provided}
  syncpack prompt --filter {yellow "typescript|tslint"}
  {dim # only inspect "devDependencies"}
  syncpack prompt --types dev
  {dim # only inspect "devDependencies" and "peerDependencies"}
  syncpack prompt --types dev,peer
  {dim # indent package.json with 4 spaces instead of 2}
  syncpack prompt --indent {yellow "    "}

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
  .option(...option.specs)
  .option(...option.types)
  .option(...option.indent)
  .parse(process.argv);

Effect.runPromise<never, unknown>(
  prompt({
    io,
    cli: {
      configPath: program.opts().config,
      filter: program.opts().filter,
      source: program.opts().source,
      specs: program.opts().specs,
      types: program.opts().types,
      indent: program.opts().indent,
    },
  }),
);
