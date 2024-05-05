import { expect } from 'vitest';
import type {
  PackageJson,
  PackageJsonFile,
} from '../../src/get-package-json-files/package-json-file.js';
import type { JsonFile } from '../../src/io/read-json-file-sync.js';

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
