import { removeSync } from 'fs-extra';
import { getWrappers, SourceWrapper } from '.';
import mock = require('mock-fs');
import { toJson, withJson } from '../../../../test/mock';
import { CWD } from '../../../constants';

describe('getWrappers', () => {
  afterEach(() => {
    mock.restore();
  });

  beforeAll(() => {
    console.log('https://github.com/tschaub/mock-fs/issues/234');
  });

  beforeEach(() => {
    mock({
      'package.json': toJson({ name: 'root' }),
      'cli/a/package.json': toJson({ name: 'cli-a' }),
      'cli/b/package.json': toJson({ name: 'cli-b' }),
      'lerna.json': toJson({ packages: ['lerna/*'] }),
      'lerna/a/package.json': toJson({ name: 'lerna-a' }),
      'lerna/b/package.json': toJson({ name: 'lerna-b' }),
      'packages/a/package.json': toJson({ name: 'packages-a' }),
      'packages/b/package.json': toJson({ name: 'packages-b' }),
    });
  });

  const getShape = (name: string, filePath: string): SourceWrapper =>
    withJson({
      contents: { name },
      filePath: `${CWD}/${filePath}`,
    });

  it('prefers CLI', () => {
    const program = { source: ['cli/*/package.json'] };
    expect(getWrappers(program)).toEqual([
      getShape('cli-a', 'cli/a/package.json'),
      getShape('cli-b', 'cli/b/package.json'),
    ]);
  });

  it('returns empty array when no patterns match', () => {
    const program = { source: ['typo.json'] };
    expect(getWrappers(program)).toEqual([]);
  });

  it('falls back to lerna.json if present', () => {
    const program = { source: [] };
    expect(getWrappers(program)).toEqual([
      getShape('root', 'package.json'),
      getShape('lerna-a', 'lerna/a/package.json'),
      getShape('lerna-b', 'lerna/b/package.json'),
    ]);
  });

  it('resorts to defaults', () => {
    const program = { source: [] };
    removeSync('lerna.json');
    expect(getWrappers(program)).toEqual([
      getShape('root', 'package.json'),
      getShape('packages-a', 'packages/a/package.json'),
      getShape('packages-b', 'packages/b/package.json'),
    ]);
  });

  describe('yarn workspaces', () => {
    afterEach(() => {
      mock.restore();
    });

    describe('when configuration is an array', () => {
      beforeEach(() => {
        mock({
          'package.json': toJson({ workspaces: ['as-array/*'] }),
          'as-array/a/package.json': toJson({ name: 'packages-a' }),
          'as-array/b/package.json': toJson({ name: 'packages-b' }),
        });
      });

      it('should resolve correctly', () => {
        const program = { source: [] };
        expect(getWrappers(program)).toEqual([
          withJson({ contents: { workspaces: ['as-array/*'] }, filePath: `${CWD}/package.json` }),
          getShape('packages-a', 'as-array/a/package.json'),
          getShape('packages-b', 'as-array/b/package.json'),
        ]);
      });
    });

    describe('when configuration is an object', () => {
      beforeEach(() => {
        mock({
          'package.json': toJson({ workspaces: { packages: ['as-object/*'] } }),
          'as-object/a/package.json': toJson({ name: 'packages-a' }),
          'as-object/b/package.json': toJson({ name: 'packages-b' }),
        });
      });

      it('should resolve correctly', () => {
        const program = { source: [] };
        expect(getWrappers(program)).toEqual([
          withJson({
            contents: { workspaces: { packages: ['as-object/*'] } },
            filePath: `${CWD}/package.json`,
          }),
          getShape('packages-a', 'as-object/a/package.json'),
          getShape('packages-b', 'as-object/b/package.json'),
        ]);
      });
    });
  });

  describe('pnpm workspaces', () => {
    afterEach(() => {
      mock.restore();
    });

    beforeEach(() => {
      mock({
        'package.json': toJson({ name: 'root' }),
        'pnpm-workspace.yaml': ['packages:', '  - "./*"'].join('\n'),
        'a/package.json': toJson({ name: 'package-a' }),
        'b/package.json': toJson({ name: 'package-b' }),
      });
    });

    it('should resolve correctly', () => {
      const program = { source: [] };
      expect(getWrappers(program)).toEqual([
        withJson({ contents: { name: 'root' }, filePath: `${CWD}/package.json` }),
        getShape('package-a', 'a/package.json'),
        getShape('package-b', 'b/package.json'),
      ]);
    });
  });
});
