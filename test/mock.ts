import { EOL } from 'os';
import { join } from 'path';
import { CWD } from '../src/constants';
import type { JsonFile } from '../src/get-package-json-files/get-patterns/read-json-safe';
import type { PackageJson } from '../src/get-package-json-files/package-json-file';
import { newlines } from '../src/lib/newlines';
import { splitNameAndVersion } from '../src/lib/split-name-and-version';

export const mockPackage = (
  dirName: string,
  {
    deps,
    devDeps,
    omitName = false,
    overrides,
    peerDeps,
    pnpmOverrides,
    resolutions,
    otherProps,
  }: {
    deps?: string[];
    devDeps?: string[];
    omitName?: boolean;
    overrides?: string[];
    peerDeps?: string[];
    pnpmOverrides?: string[];
    resolutions?: string[];
    otherProps?: Record<string, string | Record<string, any>>;
  } = {},
): JsonFile<PackageJson> => {
  return withJson({
    contents: {
      ...(!omitName ? { name: dirName } : {}),
      ...(deps && deps.length > 0
        ? {
            dependencies: toObject(deps),
          }
        : {}),
      ...(devDeps && devDeps.length > 0
        ? {
            devDependencies: toObject(devDeps),
          }
        : {}),
      ...(overrides && overrides.length > 0
        ? {
            overrides: toObject(overrides),
          }
        : {}),
      ...(peerDeps && peerDeps.length > 0
        ? {
            peerDependencies: toObject(peerDeps),
          }
        : {}),
      ...(pnpmOverrides && pnpmOverrides.length > 0
        ? {
            pnpm: {
              overrides: toObject(pnpmOverrides),
            },
          }
        : {}),
      ...(resolutions && resolutions.length > 0
        ? {
            resolutions: toObject(resolutions),
          }
        : {}),
      ...(otherProps ? otherProps : {}),
    },
    filePath: join(CWD, `${dirName}/package.json`),
  });
};

function withJson({
  contents,
  filePath,
}: {
  contents: { [key: string]: any };
  filePath: string;
}): JsonFile<PackageJson> {
  return {
    contents,
    filePath,
    json: toJson(contents),
  };
}

function toJson(contents: PackageJson): string {
  return newlines.fix(`${JSON.stringify(contents, null, 2)}${EOL}`, EOL);
}

function toObject(identifiers: string[]): { [key: string]: string } {
  return identifiers.reduce<{ [key: string]: string }>((memo, dep) => {
    const [name, version] = splitNameAndVersion(dep);
    memo[name] = version;
    return memo;
  }, {});
}
