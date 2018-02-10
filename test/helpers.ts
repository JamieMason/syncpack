import * as _ from 'lodash';
import { formatJson } from '../src/lib/write-json';
import { IDictionary, IManifest, IManifestDescriptor } from '../src/typings';

export const createFile = (
  name?: string,
  dependencies?: IManifest['dependencies'],
  devDependencies?: IManifest['devDependencies'],
  peerDependencies?: IManifest['peerDependencies']
) => formatJson(createManifest(name, dependencies, devDependencies, peerDependencies));

export const createMockFs = (
  name?: string,
  dependencies?: IManifest['dependencies'],
  devDependencies?: IManifest['devDependencies'],
  peerDependencies?: IManifest['peerDependencies']
): { [path: string]: string } => ({
  [`/Users/you/Dev/monorepo/packages/${name}/package.json`]: createFile(
    name,
    dependencies,
    devDependencies,
    peerDependencies
  )
});

export const createMockDescriptor = (
  name?: string,
  dependencies?: IManifest['dependencies'],
  devDependencies?: IManifest['devDependencies'],
  peerDependencies?: IManifest['peerDependencies']
): IManifestDescriptor => ({
  data: createManifest(name, dependencies, devDependencies, peerDependencies),
  path: `/Users/you/Dev/monorepo/packages/${name}/package.json`
});

export const createManifest = (
  name?: string,
  dependencies?: IManifest['dependencies'],
  devDependencies?: IManifest['devDependencies'],
  peerDependencies?: IManifest['peerDependencies']
): IManifest => {
  const manifest = { dependencies, devDependencies, name, peerDependencies };
  if (!dependencies) {
    delete manifest.dependencies;
  }
  if (!devDependencies) {
    delete manifest.devDependencies;
  }
  if (!name) {
    delete manifest.name;
  }
  if (!peerDependencies) {
    delete manifest.peerDependencies;
  }
  return manifest;
};

const shuffle = (value: any): typeof value =>
  _.isArray(value) ? _.shuffle(value) : _.isObject(value) ? shuffleObject(value) : value;

export const shuffleObject = (obj: object): object =>
  _(obj)
    .entries()
    .map(([key, value]) => [key, shuffle(value)])
    .shuffle()
    .reduce((next, [key, value]) => ({ ...next, [key]: value }), {});
