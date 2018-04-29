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
  versions
    .concat()
    .sort()
    .sort((a: string, b: string) => {
      if (a === '*') {
        return GREATER;
      }
      if (b === '*') {
        return LESSER;
      }
      if (a === b) {
        return SAME;
      }
      let aRange = getVersionRange(a);
      let bRange = getVersionRange(b);
      let aNumber = getVersionNumber(a);
      let bNumber = getVersionNumber(b);
      if (aNumber.indexOf('.x') !== -1) {
        aNumber = aNumber.split('.x').join('.0');
        aRange = '^';
      }
      if (bNumber.indexOf('.x') !== -1) {
        bNumber = bNumber.split('.x').join('.0');
        bRange = '^';
      }
      if (semver.gt(aNumber, bNumber)) {
        return GREATER;
      }
      if (semver.lt(aNumber, bNumber)) {
        return LESSER;
      }
      if (SEMVER_ORDER.indexOf(aRange) > SEMVER_ORDER.indexOf(bRange)) {
        return GREATER;
      }
      if (SEMVER_ORDER.indexOf(aRange) < SEMVER_ORDER.indexOf(bRange)) {
        return LESSER;
      }
      return SAME;
    });

export const getNewest: GetNewest = (versions: string[]) => _(sortBySemver(versions)).last();
