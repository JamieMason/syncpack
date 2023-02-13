import { R } from '@mobily/ts-belt';
import { normalize } from 'path';
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
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['yarn', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
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
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['bar', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('returns R.Error when path is not found', () => {
  const pathDef: PathDef<'name@version'> = {
    name: 'workspace',
    path: 'never.gonna',
    strategy: 'name@version',
  };
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  expect(fn.read(file, pathDef)).toEqual(
    R.Error(
      new BaseError(
        `Failed to get never.gonna in ${normalize('foo/package.json')}`,
      ),
    ),
  );
});
