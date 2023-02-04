#!/usr/bin/env node

import chalk from 'chalk';
import { program } from 'commander';
import { disk } from '../lib/disk';
import { option } from '../option';
import { lintSemverRangesCli } from './lint-semver-ranges-cli';

program.description(
  chalk`
  Check dependency versions within {yellow dependencies}, {yellow devDependencies},
  {yellow peerDependencies}, {yellow overrides}, and {yellow resolutions} follow a consistent format.`.replace(
    /^\n/,
    '',
  ),
);

program.on('--help', () => {
  console.log(chalk`
Examples:
  {dim # uses defaults for resolving packages}
  syncpack lint-semver-ranges
  {dim # uses packages defined by --source when provided}
  syncpack lint-semver-ranges --source {yellow "apps/*/package.json"}
  {dim # multiple globs can be provided like this}
  syncpack lint-semver-ranges --source {yellow "apps/*/package.json"} --source {yellow "core/*/package.json"}
  {dim # uses dependencies regular expression defined by --filter when provided}
  syncpack lint-semver-ranges --filter {yellow "typescript|tslint"}
  {dim # use ~ range instead of default ""}
  syncpack lint-semver-ranges --semver-range ~
  {dim # use ~ range in "devDependencies"}
  syncpack lint-semver-ranges --dev --semver-range ~
  {dim # use ~ range in "devDependencies" and "peerDependencies"}
  syncpack lint-semver-ranges --dev --peer --semver-range ~

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
  .option(...option.filter)
  .option(...option.semverRange)
  .option(...option.config)
  .option(...option.prod)
  .option(...option.dev)
  .option(...option.peer)
  .option(...option.resolutions)
  .option(...option.overrides)
  .option(...option.workspace)
  .parse(process.argv);

lintSemverRangesCli(
  {
    configPath: program.opts().config,
    dev: program.opts().dev,
    filter: program.opts().filter,
    overrides: program.opts().overrides,
    peer: program.opts().peer,
    pnpmOverrides: program.opts().pnpmOverrides,
    prod: program.opts().prod,
    resolutions: program.opts().resolutions,
    semverRange: program.opts().semverRange,
    source: program.opts().source,
    workspace: program.opts().workspace,
  },
  disk,
);
