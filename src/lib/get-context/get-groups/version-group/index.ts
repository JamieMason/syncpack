import type { TConfig } from '../../../../types';
import type { Instance } from '../../get-package-json-files/package-json-file/instance';
import type { InstanceGroup } from './instance-group';

export class VersionGroup {
  /** */
  dependencies: string[];
  /** Optionally limit this group to dependencies of the provided types */
  dependencyTypes?: TConfig.DependencyType.NameList;
  /** */
  input: TConfig.Private;
  /** */
  instanceGroups: InstanceGroup[];
  /** */
  instances: Instance[];
  /** */
  instancesByName: Record<string, Instance[]>;
  /** */
  isBanned: boolean;
  /** */
  isDefault: boolean;
  /** */
  isIgnored: boolean;
  /** */
  packages: string[];
  /** Optionally force all dependencies in this group to have this version */
  pinVersion?: string;

  constructor(input: TConfig.Private, versionGroup: TConfig.VersionGroup.Any) {
    type Banned = TConfig.VersionGroup.Banned;
    type Ignored = TConfig.VersionGroup.Ignored;
    type Pinned = TConfig.VersionGroup.Pinned;

    this.dependencies = versionGroup.dependencies;
    this.dependencyTypes = versionGroup.dependencyTypes;
    this.input = input;
    this.instanceGroups = [];
    this.instances = [];
    this.instancesByName = {};
    this.isBanned = (versionGroup as Banned).isBanned === true;
    this.isDefault = versionGroup === input.defaultVersionGroup;
    this.isIgnored = (versionGroup as Ignored).isIgnored === true;
    this.packages = versionGroup.packages;
    this.pinVersion = (versionGroup as Pinned).pinVersion;
  }

  getInvalidInstanceGroups(): InstanceGroup[] {
    return this.instanceGroups.filter((group) => group.isInvalid);
  }
}
