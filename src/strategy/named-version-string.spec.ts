import { Err, Ok } from 'tightrope/result';
import { mockPackage } from '../../test/mock';
import { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { NamedVersionStringStrategy } from './named-version-string';

it('gets and sets a name and version from a single string', () => {
  const strategy = new NamedVersionStringStrategy('local', 'packageManager');
  const jsonFile = mockPackage('foo', { otherProps: { packageManager: 'yarn@1.2.3' } });
  const file = new PackageJsonFile(jsonFile, {} as any);
  const initial = [['yarn', '1.2.3']];
  const updated = [['yarn', '2.0.0']];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['yarn', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('gets and sets a name and version from a single string nested location', () => {
  const strategy = new NamedVersionStringStrategy('custom', 'deeper.versionNumber');
  const jsonFile = mockPackage('foo', {
    otherProps: {
      deeper: { versionNumber: 'bar@1.2.3' },
    },
  });
  const file = new PackageJsonFile(jsonFile, {} as any);
  const initial = [['bar', '1.2.3']];
  const updated = [['bar', '2.0.0']];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['bar', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('returns new Err when path is not found', () => {
  const strategy = new NamedVersionStringStrategy('local', 'never.gonna');
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any);
  expect(strategy.read(file)).toEqual(new Err(expect.any(Error)));
});
