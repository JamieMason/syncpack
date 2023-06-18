import { isString } from 'tightrope/guard/is-string';
import { isSemver } from './is-semver';

export function isSupported(version: unknown): version is string {
  return version === '*' || isSemver(version) || isWorkspaceProtocol(version);
}
function isWorkspaceProtocol(version: unknown): boolean {
  if (!isString(version)) return false;
  if (!version.startsWith('workspace:')) return false;
  const value = version.replace(/^workspace:/, '');
  return value === '*' || isSemver(value);
}
