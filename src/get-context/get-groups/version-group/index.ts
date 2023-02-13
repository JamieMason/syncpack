import type { Syncpack } from '../../../types';
import type { Instance } from '../../get-package-json-files/package-json-file/instance';
import type { InstanceGroup } from './instance-group';

export class VersionGroup {
  /** */
  dependencies: string[];
  /** Optionally limit this group to dependencies at these named paths */
  dependencyTypes: Syncpack.TypeName[];
  /** */
  input: Syncpack.Config.Private;
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

  constructor(
    input: Syncpack.Config.Private,
    versionGroup: Syncpack.Config.VersionGroup.Any,
  ) {
    type Banned = Syncpack.Config.VersionGroup.Banned;
    type Ignored = Syncpack.Config.VersionGroup.Ignored;
    type Pinned = Syncpack.Config.VersionGroup.Pinned;

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
