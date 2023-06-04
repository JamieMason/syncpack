import { pipe } from 'tightrope/fn/pipe';
import { lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges';
import { listMismatches } from '../bin-list-mismatches/list-mismatches';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Effects } from '../lib/effects';
import { exitIfInvalid } from '../lib/exit-if-invalid';

export function lintCli(input: Partial<CliConfig>, effects: Effects): void {
  pipe(
    getContext(input, effects),
    listMismatches,
    lintSemverRanges,
    exitIfInvalid,
  );
}
