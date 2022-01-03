import chalk from 'chalk';
import { listVersionGroups } from '../bin-list/list-version-groups';
import type { Disk } from '../lib/disk';
import type { ProgramInput } from '../lib/get-input';
import { writeIfChanged } from '../lib/write-if-changed';
import { getExpectedVersion } from './get-expected-version';

export function fixMismatches(input: ProgramInput, disk: Disk): void {
  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  input.instances.versionGroups.reverse().forEach((versionGroup, i) => {
    const isVersionGroup = i > 0;
    const groups = listVersionGroups(versionGroup);

    if (isVersionGroup) {
      console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
    }

    groups.forEach(({ hasMismatches, instances, name }) => {
      if (hasMismatches) {
        const nextVersion = getExpectedVersion(name, versionGroup, input);
        instances.forEach(({ dependencyType, version, wrapper }) => {
          const root: any = wrapper.contents;
          if (version !== nextVersion) {
            root[dependencyType][name] = nextVersion;
          }
        });
      }
    });
  });

  input.wrappers.forEach((wrapper) => {
    writeIfChanged(disk, {
      contents: wrapper.contents,
      filePath: wrapper.filePath,
      indent: input.indent,
      json: wrapper.json,
    });
  });
}
