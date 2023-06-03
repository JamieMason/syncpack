import { pipe } from 'tightrope/fn/pipe';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges';
import { listMismatches } from '../bin-list-mismatches/list-mismatches';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';

export function lintCli(input: Partial<CliConfig>, disk: Disk): void {
  pipe(
    getContext(input, disk),
    listMismatches,
    lintSemverRanges,
    exitIfInvalid,
  );
}
