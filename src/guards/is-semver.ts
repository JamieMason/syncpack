import { isString } from 'tightrope/guard/is-string.js';

/** @deprecated migrate to make better use of npm-package-arg */
export function isSemver(version: unknown): boolean {
  const range = '(~|\\^|>=|>|<=|<)?';
  const ints = '[0-9]+';
  const intsOrX = '([0-9]+|x)';
  const dot = '\\.';
  const major = new RegExp(`^${range}${ints}$`);
  const minor = new RegExp(`^${range}${ints}${dot}${intsOrX}$`);
  const patch = new RegExp(`^${range}${ints}${dot}${intsOrX}${dot}${intsOrX}$`);
  return (
    isString(version) &&
    (version.search(major) !== -1 || version.search(minor) !== -1 || version.search(patch) !== -1)
  );
}
