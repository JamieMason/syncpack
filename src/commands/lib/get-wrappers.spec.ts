import { removeSync } from 'fs-extra';
import mock = require('mock-fs');
import { getWrappers, SourceWrapper } from './get-wrappers';

describe('getWrappers', () => {
  afterEach(() => {
    mock.restore();
  });

  beforeAll(() => {
    console.log('https://github.com/tschaub/mock-fs/issues/234');
  });

  beforeEach(() => {
    mock({
      'cli/a/package.json': JSON.stringify({ name: 'cli-a' }),
      'cli/b/package.json': JSON.stringify({ name: 'cli-b' }),
      'lerna.json': JSON.stringify({ packages: ['lerna/*'] }),
      'lerna/a/package.json': JSON.stringify({ name: 'lerna-a' }),
      'lerna/b/package.json': JSON.stringify({ name: 'lerna-b' }),
      'packages/a/package.json': JSON.stringify({ name: 'packages-a' }),
      'packages/b/package.json': JSON.stringify({ name: 'packages-b' }),
    });
  });

  const getShape = (name: string, filePath: string): SourceWrapper => ({ contents: { name }, filePath });

  it('prefers CLI', () => {
    const program = { source: ['cli/*/package.json'] };
    expect(getWrappers(program)).toEqual([
      getShape('cli-a', 'cli/a/package.json'),
      getShape('cli-b', 'cli/b/package.json'),
    ]);
  });

  it('falls back to lerna.json if present', () => {
    const program = { source: [] };
    expect(getWrappers(program)).toEqual([
      getShape('lerna-a', 'lerna/a/package.json'),
      getShape('lerna-b', 'lerna/b/package.json'),
    ]);
  });

  it('resorts to defaults', () => {
    const program = { source: [] };
    removeSync('lerna.json');
    expect(getWrappers(program)).toEqual([
      getShape('packages-a', 'packages/a/package.json'),
      getShape('packages-b', 'packages/b/package.json'),
    ]);
  });

  describe('yarn workspaces', () => {
    afterEach(() => {
      mock.restore();
    });

    beforeEach(() => {
      mock({
        'package.json': JSON.stringify({ workspaces: ['packages/*'] }),
        'packages/a/package.json': JSON.stringify({ name: 'packages-a' }),
        'packages/b/package.json': JSON.stringify({ name: 'packages-b' }),
      });
    });

    it('should resolve correctly', () => {
      const program = { source: [] };
      expect(getWrappers(program)).toEqual([
        getShape('packages-a', 'packages/a/package.json'),
        getShape('packages-b', 'packages/b/package.json'),
      ]);
    });
  });
});
