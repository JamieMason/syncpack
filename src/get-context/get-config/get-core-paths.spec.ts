import 'expect-more-jest';
import { getCorePaths } from './get-core-paths';

describe('getCorePaths', () => {
  const fn = getCorePaths;
  const dev = {
    name: 'dev',
    path: 'devDependencies',
    strategy: 'versionsByName',
  };

  const overrides = {
    name: 'overrides',
    path: 'overrides',
    strategy: 'versionsByName',
  };

  const peer = {
    name: 'peer',
    path: 'peerDependencies',
    strategy: 'versionsByName',
  };

  const pnpmOverrides = {
    name: 'pnpmOverrides',
    path: 'pnpm.overrides',
    strategy: 'versionsByName',
  };

  const prod = {
    name: 'prod',
    path: 'dependencies',
    strategy: 'versionsByName',
  };

  const resolutions = {
    name: 'resolutions',
    path: 'resolutions',
    strategy: 'versionsByName',
  };

  const workspace = {
    name: 'workspace',
    namePath: 'name',
    path: 'version',
    strategy: 'name~version',
  };

  const allTypes = [
    dev,
    overrides,
    peer,
    pnpmOverrides,
    prod,
    resolutions,
    workspace,
  ];

  it('includes all if none are set', () => {
    expect(
      fn({
        dev: true,
        overrides: true,
        peer: true,
        pnpmOverrides: true,
        prod: true,
        resolutions: true,
        workspace: true,
      }),
    ).toEqual(allTypes);
  });

  it('includes all if all are set', () => {
    expect(fn({})).toEqual(allTypes);
  });

  it('enables one if it is the only one set', () => {
    expect(fn({ dev: true })).toEqual([dev]);
    expect(fn({ overrides: true })).toEqual([overrides]);
    expect(fn({ peer: true })).toEqual([peer]);
    expect(fn({ pnpmOverrides: true })).toEqual([pnpmOverrides]);
    expect(fn({ prod: true })).toEqual([prod]);
    expect(fn({ resolutions: true })).toEqual([resolutions]);
    expect(fn({ workspace: true })).toEqual([workspace]);
  });

  it('enables some if only those are set', () => {
    expect(fn({ dev: true, prod: true, workspace: true })).toEqual([
      dev,
      prod,
      workspace,
    ]);
  });
});
