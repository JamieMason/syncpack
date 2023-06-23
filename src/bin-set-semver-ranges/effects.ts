import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { ICON } from '../constants';
import type {
  SemverRangeEffectInput as Input,
  SemverRangeEffects,
} from '../create-program/effects';
import type { SemverGroupReport } from '../get-semver-groups';

export const setSemverRangesEffects: SemverRangeEffects<void> = {
  onFilteredOut() {
    return Effect.unit();
  },
  onIgnored() {
    return Effect.unit();
  },
  onValid() {
    return Effect.unit();
  },
  onSemverRangeMismatch(input) {
    return Effect.sync(() => setVersions(input));
  },
  onUnsupportedVersion(input) {
    return Effect.sync(() => logUnsupportedVersion(input));
  },
  onWorkspaceSemverRangeMismatch(input) {
    return Effect.sync(() => setVersions(input));
  },
  onComplete() {
    return Effect.unit();
  },
};

function setVersions({ report }: Input<SemverGroupReport.FixableCases>) {
  report.instance.setVersion(report.expectedVersion);
}

function logUnsupportedVersion({ report }: Input<SemverGroupReport.UnsupportedVersion>) {
  console.log(
    chalk`{yellow %s} %s {yellow %s} {dim ignored as a format which syncpack cannot apply semver ranges to}`,
    ICON.panic,
    report.name,
    report.instance.version,
  );
}
