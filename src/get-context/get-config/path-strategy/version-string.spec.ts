import { R } from '@mobily/ts-belt';
import { normalize } from 'path';
import { mockPackage } from '../../../../test/mock';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { PackageJsonFile } from '../../get-package-json-files/package-json-file';
import type { PathDef } from './types';
import { versionString as fn } from './version-string';

it('gets and sets an anonymous version from a single string', () => {
  const pathDef: PathDef<'version'> = {
    name: 'workspace',
    path: 'someVersion',
    strategy: 'version',
  };
  const jsonFile = mockPackage('foo', {
    otherProps: { someVersion: '1.2.3' },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [['someVersion', '1.2.3']];
  const updated = [['someVersion', '2.0.0']];
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['someVersion', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('gets and sets an anonymous version from a single string in a nested location', () => {
  const pathDef: PathDef<'version'> = {
    name: 'custom',
    path: 'engines.node',
    strategy: 'version',
  };
  const jsonFile = mockPackage('foo', {
    otherProps: {
      engines: { node: '1.2.3' },
    },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [['node', '1.2.3']];
  const updated = [['node', '2.0.0']];
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['node', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('returns R.Error when path is not found', () => {
  const pathDef: PathDef<'version'> = {
    name: 'workspace',
    path: 'never.gonna',
    strategy: 'version',
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
