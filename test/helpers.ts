import { IDictionary, IManifest } from '../src/typings';

export const createMockProject = (
  name: string,
  dependencies: IManifest['dependencies'] = {},
  devDependencies: IManifest['devDependencies'] = {},
  peerDependencies: IManifest['peerDependencies'] = {}
): { [path: string]: string } => ({
  [`/Users/you/Dev/monorepo/packages/${name}/package.json`]: JSON.stringify(
    createManifest(name, dependencies, devDependencies, peerDependencies)
  )
});

export const createManifest = (
  name: string,
  dependencies: IManifest['dependencies'] = {},
  devDependencies: IManifest['devDependencies'] = {},
  peerDependencies: IManifest['peerDependencies'] = {}
): IManifest => ({
  dependencies,
  devDependencies,
  name,
  peerDependencies
});
