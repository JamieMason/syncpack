#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
RESULTS="$ROOT/results.json"
README="$ROOT/README.md"

if [[ ! -f "$RESULTS" ]]; then
  echo "results.json not found — run ./bench.sh first" >&2
  exit 1
fi

CHART=$(node -e "
  const data = require('$RESULTS');
  const BAR_WIDTH = 40;
  const rows = data.results.map(r => ({
    name: r.command,
    mean: r.mean,
    stddev: r.stddev,
    min: r.min,
    max: r.max,
  }));
  const maxMean = Math.max(...rows.map(r => r.mean));

  const chart = rows.map(r => {
    const len = Math.max(1, Math.round((r.mean / maxMean) * BAR_WIDTH));
    const bar = '█'.repeat(len);
    return r.name.padStart(8) + ' │ ' + bar + ' ' + r.mean.toFixed(3) + 's';
  }).join('\n');

  console.log(chart);
")

TABLE=$(node -e "
  const data = require('$RESULTS');
  const rows = data.results.map(r =>
    '| ' + r.command.padEnd(8) + '| ' +
    r.mean.toFixed(3).padStart(8) + ' | ' +
    r.stddev.toFixed(3).padStart(7) + ' | ' +
    r.min.toFixed(3).padStart(7) + ' | ' +
    r.max.toFixed(3).padStart(7) + ' |'
  ).join('\n');
  console.log(rows);
")

cat > "$README" << EOF
# syncpack benchmarks

Benchmark comparing \`syncpack list\` performance across versions, run against the [Fluid Framework](https://github.com/microsoft/FluidFramework) monorepo.

## Results

\`\`\`
syncpack list — seconds (lower is better)

${CHART}
\`\`\`

| Version | Mean (s) | ± σ (s) | Min (s) | Max (s) |
| ------- | -------: | ------: | ------: | ------: |
${TABLE}

## Usage

\`\`\`sh
npm install
./bench.sh
./update-readme.sh
\`\`\`
EOF

echo "README.md updated"
