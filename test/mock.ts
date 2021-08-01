import { EOL } from 'os';
import { SourceWrapper } from '../src/commands/lib/get-wrappers';

const toObject = (identifiers: string[]): { [key: string]: string } =>
  identifiers.reduce<{ [key: string]: string }>((memo, dep) => {
    const [name, version] = dep.split('@');
    memo[name] = version;
    return memo;
  }, {});

export const toJson = (contents: SourceWrapper['contents']): string => `${JSON.stringify(contents, null, 2)}${EOL}`;

export const withJson = ({
  contents,
  filePath,
}: {
  contents: { [key: string]: any };
  filePath: string;
}): SourceWrapper => ({
  contents,
  filePath,
  json: toJson(contents),
});

export const wrapper = (
  dirName: string,
  deps?: string[],
  devDeps?: string[],
  peerDeps?: string[],
  otherProps?: Record<string, string | Record<string, any>>,
): SourceWrapper => {
  return withJson({
    contents: {
      ...(deps && deps.length > 0 ? { dependencies: toObject(deps) } : {}),
      ...(devDeps && devDeps.length > 0 ? { devDependencies: toObject(devDeps) } : {}),
      ...(peerDeps && peerDeps.length > 0 ? { peerDependencies: toObject(peerDeps) } : {}),
      ...(otherProps ? otherProps : {}),
    },
    filePath: `/${dirName}/package.json`,
  });
};
