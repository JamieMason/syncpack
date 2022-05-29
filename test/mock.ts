import { EOL } from 'os';
import { join, normalize } from 'path';
import { CWD } from '../src/constants';
import type { Source, SourceWrapper } from '../src/lib/get-input/get-wrappers';

export function createWrapper(contents: Source): SourceWrapper {
  return withJson({ contents, filePath: join(CWD, 'some/package.json') });
}

export function toJson(contents: SourceWrapper['contents']): string {
  return `${JSON.stringify(contents, null, 2)}${EOL}`;
}

export const mockPackage = (
  dirName: string,
  {
    deps,
    devDeps,
    peerDeps,
    otherProps,
  }: {
    deps?: string[];
    devDeps?: string[];
    peerDeps?: string[];
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
      ...(peerDeps && peerDeps.length > 0
        ? {
            peerDependencies: toObject(peerDeps),
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
