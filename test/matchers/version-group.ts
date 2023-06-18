import 'expect-more-jest';
import { Instance } from '../../src/get-package-json-files/instance';
import type { VersionGroupReport } from '../../src/get-version-groups';

export function toBeBanned({ name }: Pick<VersionGroupReport.Banned, 'name'>) {
  return expect.objectContaining({
    _tag: 'Banned',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
  });
}

export function toBeFilteredOut({ name }: Pick<VersionGroupReport.FilteredOut, 'name'>) {
  return expect.objectContaining({
    _tag: 'FilteredOut',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: true,
  });
}

export function toBeHighestSemverMismatch({
  name,
  expectedVersion,
}: Pick<VersionGroupReport.HighestSemverMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'HighestSemverMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
    expectedVersion,
  });
}

export function toBeIgnored({ name }: Pick<VersionGroupReport.Ignored, 'name'>) {
  return expect.objectContaining({
    _tag: 'Ignored',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: true,
  });
}

export function toBeLowestSemverMismatch({
  name,
  expectedVersion,
}: Pick<VersionGroupReport.LowestSemverMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'LowestSemverMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
    expectedVersion,
  });
}

export function toBePinnedMismatch({
  name,
  expectedVersion,
}: Pick<VersionGroupReport.PinnedMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'PinnedMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
    expectedVersion,
  });
}

export function toBeSameRangeMismatch({
  name,
}: Pick<VersionGroupReport.SameRangeMismatch, 'name'>) {
  return expect.objectContaining({
    _tag: 'SameRangeMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
  });
}

export function toBeSnappedToMismatch({
  name,
  expectedVersion,
}: Pick<VersionGroupReport.SnappedToMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'SnappedToMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
    expectedVersion,
  });
}

export function toBeUnsupportedMismatch({
  name,
}: Pick<VersionGroupReport.UnsupportedMismatch, 'name'>) {
  return expect.objectContaining({
    _tag: 'UnsupportedMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
  });
}

export function toBeValid({ name }: Pick<VersionGroupReport.Valid, 'name'>) {
  return expect.objectContaining({
    _tag: 'Valid',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: true,
  });
}

export function toBeWorkspaceMismatch({
  name,
  expectedVersion,
}: Pick<VersionGroupReport.WorkspaceMismatch, 'name' | 'expectedVersion'>) {
  return expect.objectContaining({
    _tag: 'WorkspaceMismatch',
    name,
    instances: expect.toBeArrayIncludingOnly([expect.any(Instance)]),
    isValid: false,
    expectedVersion,
    workspaceInstance: expect.any(Instance),
  });
}
