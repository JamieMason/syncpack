import chalk from 'chalk-template';
import type { Command } from 'commander';

export function showHelpOnError(program: Command) {
  program.showHelpAfterError(chalk`
1. The following options were replaced in syncpack@9.0.0:
{red
  -p, --prod              include dependencies
  -d, --dev               include devDependencies
  -P, --peer              include peerDependencies
  -R, --resolutions       include resolutions (yarn)
  -o, --overrides         include overrides (npm)
  -O, --pnpmOverrides     include overrides (pnpm)
  -w, --workspace         include locally developed package versions
}
  Instead use the new {green --types} option like so:

    {green --types dev,prod,peer}

2. In .syncpackrc, the following options were replaced:
{red
  "dev": true,
  "overrides": true,
  "peer": true,
  "pnpmOverrides": true,
  "prod": true,
  "resolutions": true,
  "workspace": true,
}
  Instead use the new {green dependencyTypes} array like so:
  {green
    "dependencyTypes": ["dev", "prod", "peer"]
}`);
}
