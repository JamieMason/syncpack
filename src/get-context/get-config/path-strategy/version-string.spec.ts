import { normalize } from 'path';
import { Err, Ok } from 'tightrope/result';
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
  expect(fn.read(file, pathDef)).toEqual(new Ok(initial));
  expect(fn.write(file, pathDef, ['someVersion', '2.0.0'])).toEqual(
    new Ok(file),
  );
  expect(fn.read(file, pathDef)).toEqual(new Ok(updated));
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
  expect(fn.read(file, pathDef)).toEqual(new Ok(initial));
  expect(fn.write(file, pathDef, ['node', '2.0.0'])).toEqual(new Ok(file));
  expect(fn.read(file, pathDef)).toEqual(new Ok(updated));
});

it('returns new Err when path is not found', () => {
  const pathDef: PathDef<'version'> = {
    name: 'workspace',
    path: 'never.gonna',
    strategy: 'version',
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
