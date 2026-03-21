#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
MODULES="$ROOT/node_modules"
FIXTURE="$ROOT/fixtures/fluid-framework"

cmd() {
  local version=$1
  local major=${version%%.*}
  if [[ $major -ge 14 ]]; then
    echo "$MODULES/syncpack-$version/index.cjs list"
  elif [[ $major -ge 13 ]]; then
    echo "$MODULES/syncpack-$version/dist/bin.js list"
  else
    echo "$MODULES/syncpack-$version/dist/bin-list/index.js"
  fi
}

VERSIONS=(
  7.0.0
  8.0.0
  9.0.0
  10.0.0
  11.2.1
  12.0.0
  12.0.1
  12.1.0
  12.2.0
  12.3.0
  12.3.1
  12.3.2
  12.3.3
  12.4.0
  13.0.0
  13.0.1
  13.0.2
  13.0.3
  13.0.4
  14.0.0
  14.0.1
  14.0.2
  14.1.0
  14.2.0
  14.2.1
)

ARGS=(--warmup 3 -i --export-json "$ROOT/results.json")
for v in "${VERSIONS[@]}"; do
  c=$(cmd "$v")
  chmod +x "${c%% *}"
  ARGS+=(-n "$v" "$c")
done

cd "$FIXTURE"
hyperfine "${ARGS[@]}"
