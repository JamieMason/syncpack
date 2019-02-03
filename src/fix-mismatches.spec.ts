import { readJsonSync } from 'fs-extra';
import * as mock from 'mock-fs';
import { getFixture, getMockCommander } from '../test/helpers';
import { run } from './fix-mismatches';
import { IManifest } from './typings';

describe('fix-mismatches', () => {
  let spyConsole: any;

  afterAll(() => {
    mock.restore();
  });

  beforeAll(async () => {
    const [one, two, three] = getFixture('exact').data as IManifest[];
    const sources = [
      '/path/1/package.json',
      '/path/2/package.json',
      '/path/3/package.json'
    ];
    const program = getMockCommander(sources);
    mock({
      '/path/1/package.json': JSON.stringify(one),
      '/path/2/package.json': JSON.stringify(two),
      '/path/3/package.json': JSON.stringify(three)
    });
    const noop = () => undefined;
    spyConsole = jest.spyOn(console, 'log').mockImplementation(noop);
    await run(program);
  });

  it('sets the version of dependencies with different versions to the newest of those versions found', () => {
    expect(readJsonSync('/path/1/package.json')).toEqual(
      expect.objectContaining({
        dependencies: { chalk: '2.3.0', commander: '2.13.0' },
        devDependencies: {
          jest: '22.1.4',
          prettier: '1.10.2',
          rimraf: '2.6.2'
        },
        peerDependencies: { gulp: '*' }
      })
    );
    expect(readJsonSync('/path/2/package.json')).toEqual(
      expect.objectContaining({
        dependencies: { chalk: '2.3.0' },
        devDependencies: { jest: '22.1.4' }
      })
    );
    expect(readJsonSync('/path/3/package.json')).toEqual(
      expect.objectContaining({
        devDependencies: {
          npm: 'https://github.com/npm/npm.git',
          prettier: '1.10.2'
        },
        peerDependencies: { gulp: '*' }
      })
    );
  });
});
