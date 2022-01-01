import type { SourceWrapper } from '../../lib/get-input/get-wrappers';

/**
 * If the dependency `name` is a package developed locally in this monorepo, we
 * should use its version as the source of truth.
 */
export function getWorkspaceVersion(
  name: string,
  wrappers: SourceWrapper[],
): string {
  const wrapper = wrappers.find(({ contents }) => contents.name === name);
  if (!wrapper) return '';
  return wrapper.contents.version || '';
}
