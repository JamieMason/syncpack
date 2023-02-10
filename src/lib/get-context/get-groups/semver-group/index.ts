import type { TConfig } from '../../../../types';
import type { Instance } from '../../get-package-json-files/package-json-file/instance';

export class SemverGroup {
  /** */
  dependencies: string[];
  /** Optionally limit this group to dependencies of the provided types */
  dependencyTypes?: TConfig.DependencyType.NameList;
  /** */
  input: TConfig.Private;
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
  range: TConfig.SemverRange.Value;

  constructor(input: TConfig.Private, semverGroup: TConfig.SemverGroup.Any) {
    type Ignored = TConfig.SemverGroup.Ignored;
    type WithRange = TConfig.SemverGroup.WithRange;

    this.dependencies = semverGroup.dependencies;
    this.input = input;
    this.instances = [];
    this.instancesByName = {};
    this.isDefault = semverGroup === input.defaultSemverGroup;
    this.isIgnored = (semverGroup as Ignored).isIgnored === true;
    this.packages = semverGroup.packages;
    this.range = (semverGroup as WithRange).range;

    this.isMismatch = this.isMismatch.bind(this);
  }

  /** Does this `Instance` not follow the rules of this group? */
  isMismatch(instance: Instance) {
    return !instance.hasRange(this.range);
  }

  /** 1+ `Instance` has a version which does not follow the rules */
  hasMismatches() {
    return !this.isIgnored && this.instances.some(this.isMismatch);
  }

  /** Get every `Instance` with a version which does not follow the rules */
  getMismatches(): [string, Instance[]][] {
    return Object.entries(this.instancesByName)
      .filter(([, instances]) => instances.some(this.isMismatch))
      .map(([name, instances]) => [name, instances.filter(this.isMismatch)]);
  }
}
