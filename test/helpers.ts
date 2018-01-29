import { IDictionary, IPackageJson } from '../src/typings';

export const createMockProject = (
  name: string,
  dependencies: IPackageJson['dependencies'] = {},
  devDependencies: IPackageJson['devDependencies'] = {},
  peerDependencies: IPackageJson['peerDependencies'] = {}
): { [path: string]: string } => ({
  [`/Users/you/Dev/monorepo/packages/${name}/package.json`]: JSON.stringify(
    createPackage(name, dependencies, devDependencies, peerDependencies)
  )
});

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
