import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { ICON } from '../constants';
import type {
  SemverRangeEffectInput as Input,
  SemverRangeEffects,
} from '../create-program/effects';
import type { SemverGroupReport } from '../get-semver-groups';

export const setSemverRangesEffects: SemverRangeEffects = {
  FilteredOut() {
    return Effect.unit();
  },
  Ignored() {
    return Effect.unit();
  },
  Valid() {
    return Effect.unit();
  },
  SemverRangeMismatch(input) {
    return Effect.sync(() => setVersions(input));
  },
  UnsupportedVersion(input) {
    return Effect.sync(() => logUnsupportedVersion(input));
  },
  WorkspaceSemverRangeMismatch(input) {
    return Effect.sync(() => setVersions(input));
  },
  TearDown() {
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
