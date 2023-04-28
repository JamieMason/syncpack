import { uniq } from 'tightrope/array/uniq';
import type { Instance } from '../../get-package-json-files/instance';

export function getUniqueVersions(instances: Instance[]): string[] {
  return uniq(instances.map((i) => i.version));
}
