import * as _ from 'lodash';
import * as semver from 'semver';
import { GREATER, LESSER, SAME, SEMVER_ORDER } from './constants';

export type GetNewest = (versions: string[]) => string | undefined;
export type GetVersionNumber = (version: string) => string;
export type GetVersionRange = (version: string) => string;
export type SortBySemver = (versions: string[]) => string[];

export const getVersionNumber: GetVersionNumber = (version) => version.slice(version.search(/[0-9]/), version.length);
export const getVersionRange: GetVersionRange = (version) => version.split(/[0-9]/)[0];

export const isValid = (version: string) => semver.valid(version) !== null;

export const sortBySemver: SortBySemver = (versions: string[]) =>
  versions.concat().sort((a: string, b: string) => {
    const ar = getVersionRange(a);
    const br = getVersionRange(b);
    const ac = getVersionNumber(a);
    const bc = getVersionNumber(b);
    if (a === '*') {
      return GREATER;
    }
    if (b === '*') {
      return LESSER;
    }
    if (semver.gtr(ac, bc)) {
      return GREATER;
    }
    if (semver.ltr(ac, bc)) {
      return LESSER;
    }
    if (SEMVER_ORDER.indexOf(ar) > SEMVER_ORDER.indexOf(br)) {
      return GREATER;
    }
    if (SEMVER_ORDER.indexOf(ar) < SEMVER_ORDER.indexOf(br)) {
      return LESSER;
    }
    return SAME;
  });

export const getNewest: GetNewest = (versions: string[]) => _(sortBySemver(versions)).last();
