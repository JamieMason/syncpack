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

export type GetMismatchedFileVersions = (pattern: string) => Promise<IDictionary<string[]>>;
export type GetFileVersions = (pattern: string) => Promise<IDictionary<string[]>>;
export type SetFileVersion = (name: string, version: string, pattern: string) => Promise<IPackageJson[]>;
export type SetFileVersionRange = (range: string, pattern: string) => Promise<IPackageJson[]>;
export type SetFileVersionsToNewestMismatch = (pattern: string) => Promise<IPackageJson[]>;

export type GetMismatchedPackageVersions = (manifests: IPackageJson[]) => IDictionary<string[]>;
export type GetPackageVersions = (manifests: IPackageJson[]) => IDictionary<string[]>;
export type SetPackageVersion = (name: string, version: string, manifests: IPackageJson[]) => IPackageJson[];
export type SetPackageVersionRange = (range: string, manifests: IPackageJson[]) => IPackageJson[];
export type SetPackageVersionsToNewestMismatch = (manifests: IPackageJson[]) => IPackageJson[];

export type GetNewest = (versions: string[]) => string | undefined;
export type GetVersionNumber = (version: string) => string;
export type GetVersionRange = (version: string) => string;
export type SortBySemver = (versions: string[]) => string[];
