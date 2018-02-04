import { getUntidyManifest } from '../../../test/fixtures';
import { shuffleObject } from '../../../test/helpers';
import { SORT_FIRST } from '../../constants';
import { IManifest } from '../../typings';
import { manifestData } from './index';

describe('format', () => {
  let results: IManifest[];
  beforeAll(() => {
    results = manifestData.format([shuffleObject(getUntidyManifest()) as IManifest]);
  });

  it('sorts specified keys to the top of package.json', () => {
    results.forEach((result) => {
      expect(Object.keys(result).slice(0, SORT_FIRST.length)).toEqual(SORT_FIRST);
    });
  });

  it('sorts remaining keys alphabetically', () => {
    results.forEach((result) => {
      expect(Object.keys(result).slice(SORT_FIRST.length - 1)).toEqual([
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
        'scripts'
      ]);
    });
  });

  it('sorts "dependencies" alphabetically', () => {
    results.forEach((result) => {
      expect(result.dependencies).toEqual({
        arnold: '5.0.0',
        dog: '2.13.0',
        guybrush: '7.1.1',
        mango: '2.3.0'
      });
    });
  });

  it('sorts "devDependencies" alphabetically', () => {
    results.forEach((result) => {
      expect(result.devDependencies).toEqual({ stroopwafel: '4.4.2', waldorf: '22.1.4' });
    });
  });

  it('sorts "files" alphabetically', () => {
    results.forEach((result) => {
      expect(result.files).toEqual(['assets', 'dist']);
    });
  });

  it('sorts "keywords" alphabetically', () => {
    results.forEach((result) => {
      expect(result.keywords).toEqual(['thing', 'those', 'whatsits']);
    });
  });

  it('sorts "peerDependencies" alphabetically', () => {
    results.forEach((result) => {
      expect(result.peerDependencies).toEqual({ giftwrap: '0.1.2', jambalaya: '6.1.4', zoolander: '1.4.25' });
    });
  });

  it('sorts "scripts" alphabetically', () => {
    results.forEach((result) => {
      expect(result.scripts).toEqual({ build: 'tsc', format: 'prettier', lint: 'tslint', test: 'jest' });
    });
  });

  it('uses shorthand "bugs"', () => {
    results.forEach((result) => {
      expect(result.bugs).toEqual('https://github.com/JaneDoe/do-it/issues');
    });
  });

  it('uses shorthand "repository"', () => {
    results.forEach((result) => {
      expect(result.repository).toEqual('JaneDoe/do-it');
    });
  });
});
