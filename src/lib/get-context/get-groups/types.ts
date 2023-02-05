import type { Instance } from '../get-package-json-files/package-json-file/instance';

export type Group<T> = T & {
  instances: Instance[];
  instancesByName: Record<string, Instance[]>;
  isDefault: boolean;
};
