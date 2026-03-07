#!/usr/bin/env bash

find . -type f | while read -r file; do
  filename=$(basename "$file")
  if [[ "$filename" != "pnpm-workspace.yaml" &&
    "$filename" != "lerna.json" &&
    "$filename" != ".syncpackrc" &&
    "$filename" != ".syncpackrc.cjs" &&
    "$filename" != ".syncpackrc.js" &&
    "$filename" != ".syncpackrc.json" &&
    "$filename" != ".syncpackrc.mjs" &&
    "$filename" != ".syncpackrc.ts" &&
    "$filename" != ".syncpackrc.yaml" &&
    "$filename" != ".syncpackrc.yml" &&
    "$filename" != "syncpack.config.cjs" &&
    "$filename" != "syncpack.config.js" &&
    "$filename" != "syncpack.config.mjs" &&
    "$filename" != "syncpack.config.ts" &&
    "$filename" != "package.json" ]]; then
    rm "$file"
  fi
done

find . -type d -empty -delete
