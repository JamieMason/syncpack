import { readJsonSync } from 'fs-extra';
import { getFixture, getMockCommander, shuffleObject } from '../test/helpers';
import { SORT_FIRST } from './constants';
import { run } from './format';
import { IManifest } from './typings';
import mock = require('mock-fs');

describe('format', () => {
  let minimal: IManifest;
  let untidy: IManifest;

  beforeAll(async () => {
    const program = getMockCommander([
      '/minimal/package.json',
      '/untidy/package.json',
    ]);
    mock({
      '/minimal/package.json': JSON.stringify({
        name: 'minimal',
        version: '0.0.0',
      }),
      '/untidy/package.json': getFixture('untidy', shuffleObject).json,
    });
    await run(program);
    minimal = readJsonSync('/minimal/package.json');
    untidy = readJsonSync('/untidy/package.json');
    mock.restore();
  });

  it('sorts specified keys to the top of package.json', () => {
    expect(Object.keys(untidy).slice(0, SORT_FIRST.length)).toEqual(SORT_FIRST);
  });

  it('sorts remaining keys alphabetically', () => {
    expect(Object.keys(untidy).slice(SORT_FIRST.length - 1)).toEqual([
      'author',
      'bin',
      'bugs',
      'dependencies',
      'devDependencies',
      'files',
      'homepage',
      'keywords',
      'license',
      'main',
      'peerDependencies',
      'repository',
      'scripts',
    ]);
  });

  it('sorts "dependencies" alphabetically', () => {
    expect(minimal).not.toHaveProperty('dependencies');
    expect(untidy.dependencies).toEqual({
      arnold: '5.0.0',
      dog: '2.13.0',
      guybrush: '7.1.1',
      mango: '2.3.0',
    });
  });

  it('sorts "devDependencies" alphabetically', () => {
    expect(minimal).not.toHaveProperty('devDependencies');
    expect(untidy.devDependencies).toEqual({
      stroopwafel: '4.4.2',
      waldorf: '22.1.4',
    });
  });

  it('sorts "files" alphabetically', () => {
    expect(minimal).not.toHaveProperty('files');
    expect(untidy.files).toEqual(['assets', 'dist']);
  });

  it('sorts "keywords" alphabetically', () => {
    expect(minimal).not.toHaveProperty('keywords');
    expect(untidy.keywords).toEqual(['thing', 'those', 'whatsits']);
  });

  it('sorts "peerDependencies" alphabetically', () => {
    expect(minimal).not.toHaveProperty('peerDependencies');
    expect(untidy.peerDependencies).toEqual({
      giftwrap: '0.1.2',
      jambalaya: '6.1.4',
      zoolander: '1.4.25',
    });
  });

  it('sorts "scripts" alphabetically', () => {
    expect(minimal).not.toHaveProperty('scripts');
    expect(untidy.scripts).toEqual({
      build: 'tsc',
      format: 'prettier',
      lint: 'tslint',
      test: 'jest',
    });
  });

  it('uses shorthand "bugs"', () => {
    expect(minimal).not.toHaveProperty('bugs');
    expect(untidy.bugs).toEqual('https://github.com/JaneDoe/do-it/issues');
  });

  it('uses shorthand "repository"', () => {
    expect(minimal).not.toHaveProperty('repository');
    expect(untidy.repository).toEqual('JaneDoe/do-it');
  });
});
