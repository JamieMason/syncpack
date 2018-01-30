export interface IDictionary<T> {
  [key: string]: T;
}

export type IManifestKey = 'dependencies' | 'devDependencies' | 'peerDependencies';

export interface IManifest {
  dependencies: IDictionary<string>;
  devDependencies: IDictionary<string>;
  peerDependencies: IDictionary<string>;
  [otherProps: string]: string | IDictionary<string>;
}

export interface IManifestDescriptor {
  path: string;
  data: IManifest;
}
