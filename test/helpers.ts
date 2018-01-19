import { IDictionary, IPackageJson } from '../src/typings';

export const createPackage = (
  name: string,
  dependencies: IPackageJson['dependencies'] = {},
  devDependencies: IPackageJson['devDependencies'] = {},
  peerDependencies: IPackageJson['peerDependencies'] = {}
): IPackageJson => ({
  dependencies,
  devDependencies,
  name,
  peerDependencies
});
