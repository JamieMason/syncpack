import { CommanderStatic } from 'commander';

export interface IDictionary<T> {
  [key: string]: T;
}

export type IManifestKey =
  | 'dependencies'
  | 'devDependencies'
  | 'peerDependencies';

export interface IManifest {
  dependencies: IDictionary<string>;
  devDependencies: IDictionary<string>;
  peerDependencies: IDictionary<string>;
  [otherProps: string]: string | IDictionary<string>;
}

export interface IFileDescriptor {
  path: string;
  data: object;
}

export interface IManifestDescriptor {
  path: string;
  data: IManifest;
}

// export interface IMockCommander {
//   command: (...args: any[]) => jest.SpyInstance;
//   option: (...args: any[]) => jest.SpyInstance;
//   parse: (...args: any[]) => jest.SpyInstance;
//   source: string[];
// }

export type CommanderApi = CommanderStatic; // | IMockCommander;
