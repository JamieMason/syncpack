#!/usr/bin/env bash

function check_options {
  command_name=$1
  node dist/bin.js $command_name
  node dist/bin.js $command_name --filter "."
  node dist/bin.js $command_name --source package.json
  node dist/bin.js $command_name --types dev
  node dist/bin.js $command_name --types dev,workspace
}

check_options "fix-mismatches"
check_options "lint-semver-ranges"
check_options "list-mismatches"
check_options "list"
check_options "set-semver-ranges"

node dist/bin.js format
node dist/bin.js format --source package.json
node dist/bin.js format --indent "    "

node dist/bin.js lint-semver-ranges --semver-range ""

node dist/bin.js set-semver-ranges --semver-range ""
