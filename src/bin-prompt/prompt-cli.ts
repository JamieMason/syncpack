import chalk from 'chalk';

import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import { getVersionGroups } from '../get-version-groups';
import { getUniqueVersions } from '../get-version-groups/lib/get-unique-versions';
import type { Disk } from '../lib/disk';
import { sortByName } from '../lib/sort-by-name';
import { writeIfChanged } from '../lib/write-if-changed';

export async function promptCli(
  input: Partial<CliConfig>,
  disk: Disk,
): Promise<void> {
  const ctx = getContext(input, disk);
  const versionGroups = getVersionGroups(ctx);

  for (const versionGroup of versionGroups) {
    const reports = versionGroup.inspect().sort(sortByName);
    for (const report of reports) {
      switch (report.status) {
        case 'SAME_RANGE_MISMATCH':
        case 'UNSUPPORTED_MISMATCH': {
          const OTHER = chalk.dim('Other');
          const SKIP = chalk.dim('Skip this dependency');
          const chosenVersion = await disk.askForChoice({
            message: chalk`${report.name} {dim Choose a version to replace the others}`,
            choices: [...getUniqueVersions(report.instances), OTHER, SKIP],
          });

          if (chosenVersion === SKIP) {
            continue;
          } else if (chosenVersion === OTHER) {
            const newVersion = await disk.askForInput({
              message: chalk`${report.name} {dim Enter a new version to replace the others}`,
            });
            report.instances.forEach((instance) => {
              instance.setVersion(newVersion);
            });
          } else {
            report.instances.forEach((instance) => {
              instance.setVersion(chosenVersion);
            });
          }
        }
      }
    }
  }

  writeIfChanged(ctx);
}
