import _ = require('lodash');
import semver = require('semver');
import { GREATER, LESSER, SAME, SEMVER_ORDER } from '../constants';

const isSemver = (version: string) =>
  version === '*' ||
  version.search(/^(<|<=|~|\^|>=|>)?([0-9]+|x)\.([0-9]+|x)\.([0-9]+|x)/) !== -1;

export const getNewest = (versions: string[]): string => {
  const sorted = sortBySemver(versions);
  return sorted[sorted.length - 1];
};

export const getVersionNumber = (version: string): string =>
  version.slice(version.search(/[0-9]/), version.length);

export const getVersionRange = (version: string): string => {
  if (isSemver(version)) {
    return version.includes('.x') ? '.x' : version.split(/[0-9]/)[0];
  }
  return '';
};

export const sortBySemver = (versions: string[]): string[] =>
  versions
    .concat()
    .filter(isSemver)
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
