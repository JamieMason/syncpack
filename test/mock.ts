import { EOL } from 'os';
import { join } from 'path';
import { CWD } from '../src/constants';
import type { Source, SourceWrapper } from '../src/lib/get-input/get-wrappers';
import { setNewlines } from '../src/lib/write-if-changed/set-newlines';

export function createWrapper(contents: Source): SourceWrapper {
  return withJson({ contents, filePath: join(CWD, 'some/package.json') });
}

export function toJson(contents: SourceWrapper['contents']): string {
  return setNewlines(`${JSON.stringify(contents, null, 2)}${EOL}`, EOL);
}

export const mockPackage = (
  dirName: string,
  {
    deps,
    devDeps,
    overrides,
    peerDeps,
    pnpmOverrides,
    resolutions,
    otherProps,
  }: {
    deps?: string[];
    devDeps?: string[];
    overrides?: string[];
    peerDeps?: string[];
    pnpmOverrides?: string[];
    resolutions?: string[];
    otherProps?: Record<string, string | Record<string, any>>;
  } = {},
): SourceWrapper => {
  return withJson({
    contents: {
      name: dirName,
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
}): SourceWrapper {
  return {
    contents,
    filePath,
    json: toJson(contents),
  };
}

function toObject(identifiers: string[]): { [key: string]: string } {
  return identifiers.reduce<{ [key: string]: string }>((memo, dep) => {
    const [name, version] = dep.split('@');
    memo[name] = version;
    return memo;
  }, {});
}
