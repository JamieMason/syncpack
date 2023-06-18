import { isString } from 'tightrope/guard/is-string';
import { isSemver } from './is-semver';

export function isLooseSemver(version: unknown): boolean {
  return isString(version) && isSemver(version) && version.search(/\.x(\.|$)/) !== -1;
}
