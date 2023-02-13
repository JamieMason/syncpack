import { R } from '@mobily/ts-belt';
import { normalize } from 'path';
import { mockPackage } from '../../../../test/mock';
import { mockDisk } from '../../../../test/mock-disk';
import { BaseError } from '../../../lib/error';
import { PackageJsonFile } from '../../get-package-json-files/package-json-file';
import { nameAndVersionProps as fn } from './name-and-version-props';
import type { PathDef } from './types';

it('gets and sets a name and version from 2 seperate locations', () => {
  const pathDef: PathDef<'name~version'> = {
    name: 'workspace',
    namePath: 'name',
    path: 'version',
    strategy: 'name~version',
  };
  const jsonFile = mockPackage('foo', { otherProps: { version: '1.2.3' } });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [['foo', '1.2.3']];
  const updated = [['foo', '2.0.0']];
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['foo', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('gets and sets a name and version from 2 seperate nested locations', () => {
  const pathDef: PathDef<'name~version'> = {
    name: 'custom',
    namePath: 'sibling.id',
    path: 'deeper.versionNumber',
    strategy: 'name~version',
  };
  const jsonFile = mockPackage('foo', {
    otherProps: {
      sibling: { id: 'some-name' },
      deeper: { versionNumber: '1.2.3' },
    },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  const initial = [['some-name', '1.2.3']];
  const updated = [['some-name', '2.0.0']];
  expect(fn.read(file, pathDef)).toEqual(R.Ok(initial));
  expect(fn.write(file, pathDef, ['some-name', '2.0.0'])).toEqual(R.Ok(file));
  expect(fn.read(file, pathDef)).toEqual(R.Ok(updated));
});

it('returns R.Error when namePath is not found', () => {
  const pathDef: PathDef<'name~version'> = {
    name: 'workspace',
    namePath: 'never.gonna',
    path: 'version',
    strategy: 'name~version',
  };
  const jsonFile = mockPackage('foo', { otherProps: { version: '1.2.3' } });
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  expect(fn.read(file, pathDef)).toEqual(
    R.Error(
      new BaseError(
        `Strategy<name~version> failed to get never.gonna in ${normalize(
          'foo/package.json',
        )}`,
      ),
    ),
  );
});

it('returns R.Error when version (path) is not found', () => {
  const pathDef: PathDef<'name~version'> = {
    name: 'workspace',
    namePath: 'name',
    path: 'never.gonna',
    strategy: 'name~version',
  };
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any, mockDisk());
  expect(fn.read(file, pathDef)).toEqual(
    R.Error(
      new BaseError(
        `Strategy<name~version> failed to get never.gonna in ${normalize(
          'foo/package.json',
        )}`,
      ),
    ),
  );
});
