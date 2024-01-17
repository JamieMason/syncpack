#!/usr/bin/env node

import chalk from 'chalk-template';
import { program } from 'commander';
import { Effect } from 'effect';
import { io } from '../io/index.js';
import { showHelpOnError } from '../lib/show-help-on-error.js';
import { option } from '../option.js';
import { format } from './format.js';

program.description(
  chalk`
  Organise package.json files according to a conventional format, where fields
  appear in a predictable order and nested fields are ordered alphabetically.
  Shorthand properties are used where available, such as the {yellow repository} and
  {yellow bugs} fields.`.replace(/^\n/, ''),
);

program.on('--help', () => {
  console.log(chalk`
Examples:
  {dim # uses defaults for resolving packages}
  syncpack format
  {dim # uses packages defined by --source when provided}
  syncpack format --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack format --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # indent package.json with 4 spaces instead of 2}
  syncpack format --indent {yellow "    "}

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
  .option(...option.config)
  .option(...option.indent)
  .parse(process.argv);

Effect.runPromise<never, unknown>(
  format({
    io,
    cli: {
      configPath: program.opts().config,
      indent: program.opts().indent,
      source: program.opts().source,
    },
  }),
);
