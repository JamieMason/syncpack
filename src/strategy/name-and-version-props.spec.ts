import { Err, Ok } from 'tightrope/result';
import { mockPackage } from '../../test/mock';
import { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { NameAndVersionPropsStrategy } from './name-and-version-props';

it('gets and sets a name and version from 2 seperate locations', () => {
  const strategy = new NameAndVersionPropsStrategy('workspace', 'version', 'name');
  const jsonFile = mockPackage('foo', { otherProps: { version: '1.2.3' } });
  const file = new PackageJsonFile(jsonFile, {} as any);
  const initial = [['foo', '1.2.3']];
  const updated = [['foo', '2.0.0']];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['foo', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('gets and sets a name and version from 2 seperate nested locations', () => {
  const strategy = new NameAndVersionPropsStrategy('custom', 'deeper.versionNumber', 'sibling.id');
  const jsonFile = mockPackage('foo', {
    otherProps: {
      sibling: { id: 'some-name' },
      deeper: { versionNumber: '1.2.3' },
    },
  });
  const file = new PackageJsonFile(jsonFile, {} as any);
  const initial = [['some-name', '1.2.3']];
  const updated = [['some-name', '2.0.0']];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['some-name', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('returns new Err when namePath is not found', () => {
  const strategy = new NameAndVersionPropsStrategy('workspace', 'version', 'never.gonna');
  const jsonFile = mockPackage('foo', { otherProps: { version: '1.2.3' } });
  const file = new PackageJsonFile(jsonFile, {} as any);
  expect(strategy.read(file)).toEqual(new Err(expect.any(Error)));
});

it('returns new Err when version (path) is not found', () => {
  const strategy = new NameAndVersionPropsStrategy('workspace', 'never.gonna', 'name');
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any);
  expect(strategy.read(file)).toEqual(new Err(expect.any(Error)));
});
