import { SourceWrapper } from '../src/commands/lib/get-wrappers';

const toObject = (identifiers: string[]): { [key: string]: string } =>
  identifiers.reduce<{ [key: string]: string }>((memo, dep) => {
    const [name, version] = dep.split('@');
    memo[name] = version;
    return memo;
  }, {});

export const wrapper = (dirName: string, deps?: string[], devDeps?: string[], peerDeps?: string[]): SourceWrapper => {
  return {
    contents: {
      ...(deps && deps.length > 0 ? { dependencies: toObject(deps) } : {}),
      ...(devDeps && devDeps.length > 0 ? { devDependencies: toObject(devDeps) } : {}),
      ...(peerDeps && peerDeps.length > 0 ? { peerDependencies: toObject(peerDeps) } : {}),
    },
    filePath: `/${dirName}/package.json`,
  };
};
