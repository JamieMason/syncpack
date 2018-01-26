export type CopyValues = (options: { keys: string[]; packagesPattern: string; sourcePattern: string }) => Promise<void>;
export type SyncVersions = (options: { packagesPattern: string }) => Promise<void>;

export interface IDictionary<T> {
  [key: string]: T;
}

export type IPackageJsonKey = 'dependencies' | 'devDependencies' | 'peerDependencies';
export interface IPackageJson {
  dependencies: IDictionary<string>;
  devDependencies: IDictionary<string>;
  peerDependencies: IDictionary<string>;
  [otherProps: string]: string | IDictionary<string>;
}

export interface IPackage {
  location: string;
  json: IPackageJson;
}

export type GetVersions = (manifests: IPackageJson[]) => IDictionary<string[]>;
export type GetMismatchedVersions = (manifests: IPackageJson[]) => IDictionary<string[]>;
export type SetVersion = (name: string, version: string, manifests: IPackageJson[]) => IPackageJson[];
export type SetVersionRange = (range: string, manifests: IPackageJson[]) => IPackageJson[];
export type GetVersionRange = (version: string) => string;
export type GetVersionNumber = (version: string) => string;
export type SortBySemver = (versions: string[]) => string[];
export type GetNewest = (versions: string[]) => string | undefined;
export type SetVersionsToNewestMismatch = (manifests: IPackageJson[]) => IPackageJson[];
