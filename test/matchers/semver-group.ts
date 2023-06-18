import 'expect-more-jest';
import { Instance } from '../../src/get-package-json-files/instance';
import type { SemverGroupReport } from '../../src/get-semver-groups';

export function toBeFilteredOut({ name }: Pick<SemverGroupReport.FilteredOut, 'name'>) {
  return expect.objectContaining({
    _tag: 'FilteredOut',
    name,
    instance: expect.any(Instance),
    isValid: true,
  });
}

export function toBeIgnored({ name }: Pick<SemverGroupReport.Ignored, 'name'>) {
  return expect.objectContaining({
    _tag: 'Ignored',
    name,
    instance: expect.any(Instance),
    isValid: true,
  });
}

export function toBeValid({ name }: Pick<SemverGroupReport.Valid, 'name'>) {
  return expect.objectContaining({
    _tag: 'Valid',
    name,
    instance: expect.any(Instance),
    isValid: true,
  });
}

export function toBeWorkspaceSemverRangeMismatch({
  name,
  expectedVersion,
}: Pick<SemverGroupReport.WorkspaceSemverRangeMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'WorkspaceSemverRangeMismatch',
    name,
    instance: expect.any(Instance),
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
    instance: expect.any(Instance),
    isValid: false,
    expectedVersion,
  });
}

export function toBeUnsupportedVersion({
  name,
}: Pick<SemverGroupReport.UnsupportedVersion, 'name'>) {
  return expect.objectContaining({
    _tag: 'UnsupportedVersion',
    name,
    instance: expect.any(Instance),
    isValid: false,
  });
}
