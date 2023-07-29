import 'expect-more-jest';
import type { SemverGroupReport } from '../../src/get-semver-groups';
import { toBeSupportedInstance } from './version-group';

export function toBeFilteredOut({ name }: Pick<SemverGroupReport.FilteredOut, 'name'>) {
  return expect.objectContaining({
    _tag: 'FilteredOut',
    name,
    instance: toBeSupportedInstance(),
    isValid: true,
  });
}

export function toBeIgnored({ name }: Pick<SemverGroupReport.Ignored, 'name'>) {
  return expect.objectContaining({
    _tag: 'Ignored',
    name,
    instance: toBeSupportedInstance(),
    isValid: true,
  });
}

export function toBeValid({ name }: Pick<SemverGroupReport.Valid, 'name'>) {
  return expect.objectContaining({
    _tag: 'Valid',
    name,
    instance: toBeSupportedInstance(),
    isValid: true,
  });
}

export function toBeLocalPackageSemverRangeMismatch({
  name,
  expectedVersion,
}: Pick<SemverGroupReport.LocalPackageSemverRangeMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'LocalPackageSemverRangeMismatch',
    name,
    instance: toBeSupportedInstance(),
    isValid: false,
    expectedVersion,
  });
}

export function toBeSemverRangeMismatch({
  name,
  expectedVersion,
}: Pick<SemverGroupReport.SemverRangeMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'SemverRangeMismatch',
    name,
    instance: toBeSupportedInstance(),
    isValid: false,
    expectedVersion,
  });
}

export function toBeNonSemverVersion({ name }: Pick<SemverGroupReport.NonSemverVersion, 'name'>) {
  return expect.objectContaining({
    _tag: 'NonSemverVersion',
    name,
    instance: toBeSupportedInstance(),
    isValid: false,
  });
}
