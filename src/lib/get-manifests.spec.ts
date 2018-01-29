import * as mock from 'mock-fs';
import { createMockProject, createPackage } from '../../test/helpers';
import { getManifests } from './get-manifests';

const pattern = '/Users/you/Dev/monorepo/packages/*/package.json';

afterEach(() => {
  mock.restore();
});

beforeEach(() => {
  mock({
    ...createMockProject('foo', { chalk: '1.0.0' }),
    ...createMockProject('bar', { rimraf: '0.1.2' }),
    '/Users/you/Dev/monorepo/packages/invalid/package.json': JSON.stringify({
      some: 'incorrect shape'
    })
  });
});

describe('getManifests', () => {
  it('returns all valid package.json which match the provided globs', async () => {
    const result = await getManifests(pattern);
    expect(result).toEqual([
      { dependencies: { rimraf: '0.1.2' }, devDependencies: {}, name: 'bar', peerDependencies: {} },
      { dependencies: { chalk: '1.0.0' }, devDependencies: {}, name: 'foo', peerDependencies: {} }
    ]);
  });
});