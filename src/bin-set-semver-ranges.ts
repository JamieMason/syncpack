#!/usr/bin/env node

import chalk from 'chalk';
import { setSemverRangesToDisk } from './commands/set-semver-ranges';
import { option, SyncpackConfig } from './constants';
import { getConfig } from './lib/get-config';
import { program } from 'commander';

program.description(
  `
  Ensure dependency versions used within "dependencies", "devDependencies", and
  "peerDependencies" follow a consistent format.`.replace(/^\n/, ''),
);

program.on('--help', () => {
  console.log(chalk`
Examples:
  {dim # uses defaults for resolving packages}
  syncpack set-semver-ranges
  {dim # uses packages defined by --source when provided}
  syncpack set-semver-ranges --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack set-semver-ranges --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # uses dependencies regular expression defined by --filter when provided}
  syncpack set-semver-ranges --filter {yellow "typescript|tslint"}
  {dim # use ~ range instead of default ""}
  syncpack set-semver-ranges --semver-range ~
  {dim # set ~ range in "devDependencies"}
  syncpack set-semver-ranges --dev --semver-range ~
  {dim # set ~ range in "devDependencies" and "peerDependencies"}
  syncpack set-semver-ranges --dev --peer --semver-range ~
  {dim # indent package.json with 4 spaces instead of 2}
  syncpack set-semver-ranges --indent {yellow "    "}

Supported Ranges:
  <  {dim <1.4.2}
  <= {dim <=1.4.2}
  "" {dim 1.4.2}
  ~  {dim ~1.4.2}
  ^  {dim ^1.4.2}
  >= {dim >=1.4.2}
  >  {dim >1.4.2}
  *  {dim *}

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

program
  .option(...option.source)
  .option(...option.prod)
  .option(...option.dev)
  .option(...option.peer)
  .option(...option.filter)
  .option(...option.indent)
  .option(...option.semverRange)
  .parse(process.argv);

const args = program as Partial<SyncpackConfig>;

setSemverRangesToDisk(
  getConfig({
    dev: args.dev,
    filter: args.filter,
    indent: args.indent,
    peer: args.peer,
    prod: args.prod,
    semverRange: args.semverRange,
    source: args.source,
  }),
);
