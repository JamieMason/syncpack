import { Err, Ok } from 'tightrope/result';
import { mockPackage } from '../../test/mock';
import { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { UnnamedVersionStringStrategy } from './unnamed-version-string';

it('gets and sets an anonymous version from a single string', () => {
  const strategy = new UnnamedVersionStringStrategy('local', 'someVersion');
  const jsonFile = mockPackage('foo', { otherProps: { someVersion: '1.2.3' } });
  const file = new PackageJsonFile(jsonFile, {} as any);
  const initial = [['someVersion', '1.2.3']];
  const updated = [['someVersion', '2.0.0']];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['someVersion', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('gets and sets an anonymous version from a single string in a nested location', () => {
  const strategy = new UnnamedVersionStringStrategy('custom', 'engines.node');
  const jsonFile = mockPackage('foo', {
    otherProps: {
      engines: { node: '1.2.3' },
    },
  });
  const file = new PackageJsonFile(jsonFile, {} as any);
  const initial = [['node', '1.2.3']];
  const updated = [['node', '2.0.0']];
  expect(strategy.read(file)).toEqual(new Ok(initial));
  expect(strategy.write(file, ['node', '2.0.0'])).toEqual(new Ok(file));
  expect(strategy.read(file)).toEqual(new Ok(updated));
});

it('returns new Err when path is not found', () => {
  const strategy = new UnnamedVersionStringStrategy('local', 'never.gonna');
  const jsonFile = mockPackage('foo', {});
  const file = new PackageJsonFile(jsonFile, {} as any);
  expect(strategy.read(file)).toEqual(new Err(expect.any(Error)));
});
