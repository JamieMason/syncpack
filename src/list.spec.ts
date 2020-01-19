import mock = require('mock-fs');
import { run } from './list';
import { IManifest } from './typings';
import { getFixture, getMockCommander } from '../test/helpers';

describe('list', () => {
  let spyConsole: any;

  beforeAll(async () => {
    const [one, two, three] = getFixture('exact').data as IManifest[];
    const program = getMockCommander(
      ['/path/1/package.json', '/path/2/package.json', '/path/3/package.json'],
      '^((?!ignore).)*$'
    );
    mock({
      '/path/1/package.json': JSON.stringify(one),
      '/path/2/package.json': JSON.stringify(two),
      '/path/3/package.json': JSON.stringify(three)
    });
    const noop = () => undefined;
    spyConsole = jest.spyOn(console, 'log').mockImplementation(noop);
    await run(program);
    mock.restore();
  });

  it('returns an index of dependencies used with different versions', () => {
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('chalk'),
      expect.stringContaining('2.3.0, 1.0.0')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('commander'),
      expect.stringContaining('2.13.0')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('jest'),
      expect.stringContaining('22.1.3, 22.1.4')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('prettier'),
      expect.stringContaining('1.10.2')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('rimraf'),
      expect.stringContaining('2.6.2')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('npm'),
      expect.stringContaining('https://github.com/npm/npm.git')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('gulp'),
      expect.stringContaining('0.9.1, *')
    );
    expect(spyConsole).not.toHaveBeenCalledWith(
      expect.stringContaining('ignore')
    );
  });
});
