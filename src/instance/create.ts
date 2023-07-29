import type { FileResult, HostedGitResult, RegistryResult } from 'npm-package-arg';
import npa from 'npm-package-arg';
import type { NpmPackageArgResult, WorkspaceProtocolResult } from '.';
import { Instance } from '.';
import type { Strategy } from '../config/get-custom-types';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';

export function createInstance(
  strategy: Strategy.Any,
  name: string,
  packageJsonFile: PackageJsonFile,
  specifier: string,
): Instance.Any {
  const pkgName = packageJsonFile.contents.name || 'PACKAGE_JSON_HAS_NO_NAME';
  try {
    const parsedSpecifier = parseSpecifier(name, specifier, packageJsonFile);
    switch (parsedSpecifier.type) {
      case 'file':
      case 'directory': {
        return new Instance.FileInstance({
          name,
          packageJsonFile,
          parsedSpecifier,
          pkgName,
          specifier,
          strategy,
        });
      }
      case 'remote': {
        return new Instance.UrlInstance({
          name,
          packageJsonFile,
          parsedSpecifier,
          pkgName,
          specifier,
          strategy,
        });
      }
      case 'git': {
        return 'hosted' in parsedSpecifier
          ? new Instance.HostedGitInstance({
              name,
              packageJsonFile,
              parsedSpecifier: parsedSpecifier as HostedGitResult,
              pkgName,
              specifier,
              strategy,
            })
          : new Instance.UrlInstance({
              name,
              packageJsonFile,
              parsedSpecifier,
              pkgName,
              specifier,
              strategy,
            });
      }
      case 'alias': {
        return new Instance.AliasInstance({
          name,
          packageJsonFile,
          parsedSpecifier,
          pkgName,
          specifier,
          strategy,
        });
      }
      case 'version': {
        return new Instance.VersionInstance({
          name,
          packageJsonFile,
          parsedSpecifier: parsedSpecifier as RegistryResult & { type: 'version' },
          pkgName,
          specifier,
          strategy,
        });
      }
      case 'range': {
        return new Instance.RangeInstance({
          name,
          packageJsonFile,
          parsedSpecifier: parsedSpecifier as RegistryResult & { type: 'range' },
          pkgName,
          specifier,
          strategy,
        });
      }
      case 'tag': {
        return new Instance.TagInstance({
          name,
          packageJsonFile,
          parsedSpecifier: parsedSpecifier as RegistryResult & { type: 'tag' },
          pkgName,
          specifier,
          strategy,
        });
      }
      case 'workspaceProtocol': {
        return new Instance.WorkspaceProtocolInstance({
          name,
          packageJsonFile,
          parsedSpecifier: parsedSpecifier as WorkspaceProtocolResult,
          pkgName,
          specifier,
          strategy,
        });
      }
      default: {
        return new Instance.UnsupportedInstance({
          name,
          packageJsonFile,
          parsedSpecifier,
          pkgName,
          specifier,
          strategy,
        });
      }
    }
  } catch (err) {
    return new Instance.UnsupportedInstance({
      name,
      packageJsonFile,
      parsedSpecifier: err,
      pkgName,
      specifier,
      strategy,
    });
  }
}

function parseSpecifier(
  name: string,
  specifier: string,
  packageJsonFile: PackageJsonFile,
): NpmPackageArgResult {
  if (specifier.startsWith('workspace:')) {
    const parsed = npa.resolve(
      name,
      specifier.replace('workspace:', 'file:'),
      packageJsonFile.dirPath,
    ) as FileResult;
    return {
      type: 'workspaceProtocol',
      raw: parsed.raw.replace('file:', 'workspace:'),
      name: parsed.name,
      escapedName: parsed.escapedName,
      scope: parsed.scope,
      rawSpec: parsed.rawSpec.replace('file:', 'workspace:'),
      saveSpec: parsed.saveSpec.replace('file:', 'workspace:'),
    };
  }
  return npa.resolve(name, specifier, packageJsonFile.dirPath);
}
