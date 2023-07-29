import type { Instance } from '../../instance';

export function getUniqueSpecifiers(instances: Instance.Any[]): Instance.Any[] {
  const instancesBySpecifier: Record<string, Instance.Any> = {};
  instances.forEach((instance) => {
    instancesBySpecifier[instance.specifier] = instance;
  });
  return Object.values(instancesBySpecifier);
}
