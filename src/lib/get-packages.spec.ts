import mock = require('mock-fs');
import { resolve } from 'path';
import { getMockCommander } from '../../test/helpers';
import { getPackages } from './get-packages';
import { removeSync } from 'fs-extra';

describe('getPackages', () => {
  afterEach(() => {
    mock.restore();
  });

  beforeEach(() => {
    mock({
      'cli/a/package.json': JSON.stringify({ name: 'cli-a' }),
      'cli/b/package.json': JSON.stringify({ name: 'cli-b' }),
      'lerna.json': JSON.stringify({ packages: ['lerna/*'] }),
      'lerna/a/package.json': JSON.stringify({ name: 'lerna-a' }),
      'lerna/b/package.json': JSON.stringify({ name: 'lerna-b' }),
      'packages/a/package.json': JSON.stringify({ name: 'packages-a' }),
      'packages/b/package.json': JSON.stringify({ name: 'packages-b' })
    });
  });

  it('prefers CLI', () => {
    const program = getMockCommander(['cli/*/package.json']);
    expect(getPackages(program)).toEqual([
      { data: { name: 'cli-a' }, path: 'cli/a/package.json' },
      { data: { name: 'cli-b' }, path: 'cli/b/package.json' }
    ]);
  });

  it('falls back to lerna.json if present', () => {
    const program = getMockCommander([]);
    expect(getPackages(program)).toEqual([
      { data: { name: 'lerna-a' }, path: 'lerna/a/package.json' },
      { data: { name: 'lerna-b' }, path: 'lerna/b/package.json' }
    ]);
  });

  it('resorts to defaults', () => {
    const program = getMockCommander([]);
    removeSync('lerna.json');
    expect(getPackages(program)).toEqual([
      { data: { name: 'packages-a' }, path: 'packages/a/package.json' },
      { data: { name: 'packages-b' }, path: 'packages/b/package.json' }
    ]);
  });
});
