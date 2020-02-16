import { SourceWrapper } from '../src/commands/lib/get-wrappers';

const toObject = (identifiers: string[]) =>
  identifiers.reduce((memo: any, dep) => {
    const [name, version] = dep.split('@');
    memo[name] = version;
    return memo;
  }, {});

export const wrapper = (dirName: string, deps: string[], devDeps?: string[], peerDeps?: string[]): SourceWrapper => {
  return {
    contents: {
      ...(deps ? { dependencies: toObject(deps) } : {}),
      ...(devDeps ? { devDependencies: toObject(devDeps) } : {}),
      ...(peerDeps ? { peerDependencies: toObject(peerDeps) } : {}),
    },
    filePath: `/${dirName}/package.json`,
  };
};
