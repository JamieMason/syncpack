import { normalize } from 'path';
import { Err, Ok } from 'tightrope/result';
import { mockPackage } from '../../../../test/mock';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { PackageJsonFile } from '../../get-package-json-files/package-json-file';
import { nameAndVersionString as fn } from './name-and-version-string';
import type { PathDef } from './types';

it('gets and sets a name and version from a single string', () => {
  const pathDef: PathDef<'name@version'> = {
    name: 'workspace',
    path: 'packageManager',
    strategy: 'name@version',
  };
  const jsonFile = mockPackage('foo', {
    otherProps: { packageManager: 'yarn@1.2.3' },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [['yarn', '1.2.3']];
  const updated = [['yarn', '2.0.0']];
  expect(fn.read(file, pathDef)).toEqual(new Ok(initial));
  expect(fn.write(file, pathDef, ['yarn', '2.0.0'])).toEqual(new Ok(file));
  expect(fn.read(file, pathDef)).toEqual(new Ok(updated));
});

it('gets and sets a name and version from a single string nested location', () => {
  const pathDef: PathDef<'name@version'> = {
    name: 'custom',
    path: 'deeper.versionNumber',
    strategy: 'name@version',
  };
  const jsonFile = mockPackage('foo', {
    otherProps: {
      deeper: { versionNumber: 'bar@1.2.3' },
    },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [['bar', '1.2.3']];
  const updated = [['bar', '2.0.0']];
  expect(fn.read(file, pathDef)).toEqual(new Ok(initial));
  expect(fn.write(file, pathDef, ['bar', '2.0.0'])).toEqual(new Ok(file));
  expect(fn.read(file, pathDef)).toEqual(new Ok(updated));
});

it('returns new Err when path is not found', () => {
  const pathDef: PathDef<'name@version'> = {
    name: 'workspace',
    path: 'never.gonna',
    strategy: 'name@version',
  };
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  expect(fn.read(file, pathDef)).toEqual(
    new Err(
      new BaseError(
        `Failed to get never.gonna in ${normalize('foo/package.json')}`,
      ),
    ),
  );
});
