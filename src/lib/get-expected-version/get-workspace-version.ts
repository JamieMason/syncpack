/**
 * If the dependency `name` is a package developed locally in this monorepo, we
 * should use its version as the source of truth.
 */
export function getWorkspaceVersion(
  name: string,
  packageJsonFiles: { contents: { name?: string; version?: string } }[],
): string {
  const packageJsonFile = packageJsonFiles.find(
    ({ contents }) => contents.name === name,
  );
  if (!packageJsonFile) return '';
  return packageJsonFile.contents.version || '';
}
