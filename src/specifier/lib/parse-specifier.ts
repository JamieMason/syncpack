import type { FileResult } from 'npm-package-arg';
import npa from 'npm-package-arg';
import type { PackageJsonFile } from '../../get-package-json-files/package-json-file.js';

/** Extends npm/npm-package-arg to support "workspace:*" */
export interface WorkspaceProtocolResult {
  type: 'workspaceProtocol';
  raw: string;
  name: string | null;
  escapedName: string | null;
  scope: string | null;
  rawSpec: string;
  saveSpec: string;
}

export type NpmPackageArgResult = ReturnType<typeof npa.resolve> | WorkspaceProtocolResult;

export function parseSpecifier(
  name: string,
  specifier: string,
  packageJsonFile: PackageJsonFile,
): NpmPackageArgResult {
  if (specifier === 'workspace:*' || specifier === 'workspace:~' || specifier === 'workspace:^') {
    const parsed = npa.resolve(
      name,
      packageJsonFile.jsonFile.dirPath,
      specifier.replace('workspace:', 'file:'),
    ) as FileResult;
    return {
      escapedName: parsed.escapedName,
      name: parsed.name,
      raw: parsed.raw.replace('file:', 'workspace:'),
      rawSpec: parsed.rawSpec.replace('file:', 'workspace:'),
      saveSpec: parsed.saveSpec.replace('file:', 'workspace:'),
      scope: parsed.scope,
      type: 'workspaceProtocol',
    };
  }
  return npa.resolve(name, specifier, packageJsonFile.jsonFile.dirPath);
}
