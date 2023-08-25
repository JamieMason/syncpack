import { expect } from '@jest/globals';
import 'expect-more-jest';
import { isArray } from 'tightrope/guard/is-array';
import { isObject } from 'tightrope/guard/is-object';
import type {
  PackageJson,
  PackageJsonFile,
} from '../../src/get-package-json-files/package-json-file';
import type { JsonFile } from '../../src/io/read-json-file-sync';

export function deepPartial<T extends Record<string, any>>(value: T): T {
  if (isArray(value)) {
    return value.map(deepPartial) as unknown as T;
  }
  if (isObject(value)) {
    for (const key in value) {
      value[key] = deepPartial(value[key]);
    }
    return expect.objectContaining(value) as unknown as T;
  }
  return value;
}

export const shape = {
  PackageJsonFile(expected: Partial<JsonFile<PackageJson>>) {
    return expect.objectContaining({
      jsonFile: expect.objectContaining(expected),
    }) as unknown as PackageJsonFile;
  },
  Specifier(version: string) {
    return expect.objectContaining({ raw: version });
  },
};
