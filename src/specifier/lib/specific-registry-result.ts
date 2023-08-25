import type { RegistryResult } from 'npm-package-arg';

/**
 * A helper to create specific classes for each of the possible
 * `RegistryResult` types from npm/npm-package-arg. Instead of grouping them
 * together we are being more specific
 */
export type SpecificRegistryResult<T extends RegistryResult['type'] | 'local'> = Omit<
  RegistryResult,
  'type'
> & {
  type: T;
};
