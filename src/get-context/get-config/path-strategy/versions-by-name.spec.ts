import { R } from '@mobily/ts-belt';
import { normalize } from 'path';
import { mockPackage } from '../../../../test/mock';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { PackageJsonFile } from '../../get-package-json-files/package-json-file';
import type { PathDef } from './types';
import { versionsByName as fn } from './versions-by-name';

it('gets and sets names and versions in an object', () => {
  const pathDef: PathDef<'versionsByName'> = {
    name: 'workspace',
    path: 'dependencies',
    strategy: 'versionsByName',
  };
  const jsonFile = mockPackage('foo', { deps: ['bar@1.2.3', 'baz@4.4.4'] });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [
    ['bar', '1.2.3'],
    ['baz', '4.4.4'],
  ];
  const updated = [
    ['bar', '2.0.0'],
    ['baz', '4.4.4'],
  ];
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['bar', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('gets and sets a name and version from a single string nested location', () => {
  const pathDef: PathDef<'versionsByName'> = {
    name: 'custom',
    path: 'deeper.deps',
    strategy: 'versionsByName',
  };
  const jsonFile = mockPackage('foo', {
    otherProps: { deeper: { deps: { bar: '1.2.3', baz: '4.4.4' } } },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [
    ['bar', '1.2.3'],
    ['baz', '4.4.4'],
  ];
  const updated = [
    ['bar', '2.0.0'],
    ['baz', '4.4.4'],
  ];
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['bar', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('returns R.Error when path is not found', () => {
  const pathDef: PathDef<'versionsByName'> = {
    name: 'workspace',
    path: 'never.gonna',
    strategy: 'versionsByName',
  };
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  expect(fn.read(file, pathDef)).toEqual(
    R.Error(
      new BaseError(
        `Strategy<versionsByName> failed to get never.gonna in ${normalize(
          'foo/package.json',
        )}`,
      ),
    ),
  );
});
