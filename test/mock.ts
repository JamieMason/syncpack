import { EOL } from 'os';
import { join } from 'path';
import { CWD } from '../src/constants';
import type { Source } from '../src/lib/get-context/get-wrappers';
import type { JsonFile } from '../src/lib/get-context/get-wrappers/get-patterns/read-json-safe';
import { newlines } from '../src/lib/get-context/get-wrappers/newlines';

export function createWrapper(contents: Source): JsonFile<Source> {
  return withJson({ contents, filePath: join(CWD, 'some/package.json') });
}

export function toJson(contents: Source): string {
  return newlines.fix(`${JSON.stringify(contents, null, 2)}${EOL}`, EOL);
}

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
): JsonFile<Source> => {
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
}): JsonFile<Source> {
  return {
    contents,
    filePath,
    json: toJson(contents),
  };
}

function toObject(identifiers: string[]): { [key: string]: string } {
  return identifiers.reduce<{ [key: string]: string }>((memo, dep) => {
    const ix = dep.lastIndexOf('@');
    const name = dep.slice(0, ix);
    const version = dep.slice(ix + 1);
    memo[name] = version;
    return memo;
  }, {});
}
