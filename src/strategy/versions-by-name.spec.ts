import { Err, Ok } from 'tightrope/result';
import { mockPackage } from '../../test/mock';
import { mockEffects } from '../../test/mock-effects';
import { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { VersionsByNameStrategy } from './versions-by-name';

it('gets and sets names and versions in an object', () => {
  const strategy = new VersionsByNameStrategy('workspace', 'dependencies');
  const jsonFile = mockPackage('foo', { deps: ['bar@1.2.3', 'baz@4.4.4'] });
  const file = new PackageJsonFile(jsonFile, {} as any, mockEffects());
  const initial = [
    ['bar', '1.2.3'],
    ['baz', '4.4.4'],
  ];
  const updated = [
    ['bar', '2.0.0'],
    ['baz', '4.4.4'],
  ];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['bar', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('gets and sets a name and version from a single string nested location', () => {
  const strategy = new VersionsByNameStrategy('custom', 'deeper.deps');
  const jsonFile = mockPackage('foo', {
    otherProps: { deeper: { deps: { bar: '1.2.3', baz: '4.4.4' } } },
  });
  const file = new PackageJsonFile(jsonFile, {} as any, mockEffects());
  const initial = [
    ['bar', '1.2.3'],
    ['baz', '4.4.4'],
  ];
  const updated = [
    ['bar', '2.0.0'],
    ['baz', '4.4.4'],
  ];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['bar', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('returns new Err when path is not found', () => {
  const strategy = new VersionsByNameStrategy('workspace', 'never.gonna');
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any, mockEffects());
  expect(strategy.read(file)).toEqual(new Err(expect.any(Error)));
});
