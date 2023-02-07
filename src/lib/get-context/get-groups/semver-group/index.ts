import type {
  Config,
  DependencyType,
  ValidRange,
} from '../../get-config/config';
import type { InternalConfig } from '../../get-config/internal-config';
import type { Instance } from '../../get-package-json-files/package-json-file/instance';

export class SemverGroup {
  /** */
  dependencies: string[];
  /** Optionally limit this group to dependencies of the provided types */
  dependencyTypes?: DependencyType[];
  /** */
  input: InternalConfig;
  /** */
  instances: Instance[];
  /** */
  instancesByName: Record<string, Instance[]>;
  /** */
  isDefault: boolean;
  /** Optionally force syncpack to ignore all dependencies in this group */
  isIgnored: boolean;
  /** */
  packages: string[];
  /** The semver range which dependencies in this group should use */
  range: ValidRange;

  constructor(input: InternalConfig, semverGroup: Config.SemverGroup.Any) {
    type Ignored = Config.SemverGroup.Ignored;
    type WithRange = Config.SemverGroup.WithRange;

    this.dependencies = semverGroup.dependencies;
    this.input = input;
    this.instances = [];
    this.instancesByName = {};
    this.isDefault = semverGroup === input.defaultSemverGroup;
    this.isIgnored = (semverGroup as Ignored).isIgnored === true;
    this.packages = semverGroup.packages;
    this.range = (semverGroup as WithRange).range;
  }

  /** Syncpack must report or fix this group's mismatches */
  isInvalid() {
    return !this.isIgnored && this.hasInvalidInstances();
  }

  /** 1+ `Instance` has a version which does not follow the rules */
  hasInvalidInstances(): boolean {
    return this.getInvalidInstances().length > 0;
  }

  /** Get every `Instance` with a version which does not follow the rules */
  getInvalidInstances(): Instance[] {
    return this.instances.filter(
      (instance) =>
        instance.dependencyType !== 'workspace' &&
        !instance.hasRange(this.range),
    );
  }
}
