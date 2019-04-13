import * as mock from 'mock-fs';
import { getFixture, getMockCommander } from '../test/helpers';
import { run } from './list-mismatches';
import { IManifest } from './typings';

describe('list-mismatches', () => {
  let spyConsole: any;
  let spyProcess: any;

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
    spyProcess = jest.spyOn(process, 'exit').mockImplementation(noop);
    await run(program);
    mock.restore();
  });

  it('returns an index of dependencies used with different versions', () => {
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('gulp'),
      expect.stringContaining('0.9.1, *')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('chalk'),
      expect.stringContaining('2.3.0, 1.0.0')
    );
    expect(spyConsole).toHaveBeenCalledWith(
      expect.stringContaining('jest'),
      expect.stringContaining('22.1.3, 22.1.4')
    );
    expect(spyConsole).not.toHaveBeenCalledWith(
      expect.stringContaining('ignore')
    );
  });

  it('returns an error exit code when mismatches are found', () => {
    expect(spyProcess).toHaveBeenCalledWith(1);
  });

  it('returns exit code 0 when no mismatches are found', async () => {
    const program = getMockCommander([
      '/bar/package.json',
      '/foo/package.json'
    ]);
    mock({
      '/bar/package.json': JSON.stringify({
        dependencies: { bar: '1.0.0' },
        name: 'bar'
      }),
      '/foo/package.json': JSON.stringify({
        dependencies: { bar: '1.0.0' },
        name: 'foo'
      })
    });
    spyProcess.mockClear();
    await run(program);
    mock.restore();
    expect(spyProcess).not.toHaveBeenCalled();
  });
});
