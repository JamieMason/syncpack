import { listSemverGroupMismatches } from '../bin-lint-semver-ranges/list-semver-group-mismatches';
import type { Disk } from '../lib/disk';
import type { ProgramInput } from '../lib/get-input';
import { setSemverRange } from '../lib/set-semver-range';
import { writeIfChanged } from '../lib/write-if-changed';

export const setSemverRanges = (input: ProgramInput, disk: Disk): void => {
  input.instances.semverGroups.reverse().forEach((semverGroup) => {
    const mismatches = listSemverGroupMismatches(semverGroup);
    mismatches.forEach(({ dependencyType, name, version, wrapper }) => {
      const root: any = wrapper.contents;
      root[dependencyType][name] = setSemverRange(semverGroup.range, version);
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
};
