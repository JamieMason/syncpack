#!/usr/bin/env node

import chalk from 'chalk-template';
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { lint } from './lint.js';

program.description('  lint all versions and ranges');

program.on('--help', () => {
  console.log(chalk`
Examples:
  {dim # uses config file for resolving packages}
  syncpack lint
  {dim # uses config file defined by --config when provided}
  syncpack lint --config {yellow ./config/.syncpackrc}

Resolving Packages:
  1. If using Pnpm Workspaces, read {yellow packages} from {yellow pnpm-workspace.yaml} in the root of the project.
  2. If using Yarn Workspaces, read {yellow workspaces} from {yellow package.json}.
  3. If using Lerna, read {yellow packages} from {yellow lerna.json}.
  4. Default to {yellow "package.json"} and {yellow "packages/*/package.json"}.

Reference:
  globs            {blue.underline https://github.com/isaacs/node-glob#glob-primer}
  lerna.json       {blue.underline https://github.com/lerna/lerna#lernajson}
  Yarn Workspaces  {blue.underline https://yarnpkg.com/lang/en/docs/workspaces}
  Pnpm Workspaces  {blue.underline https://pnpm.js.org/en/workspaces}
`);
});

showHelpOnError(program);

program.option(...option.config).parse(process.argv);

Effect.runPromise<never, unknown>(
  lint({
    io,
    cli: {
      configPath: program.opts().config,
    },
  }),
);
