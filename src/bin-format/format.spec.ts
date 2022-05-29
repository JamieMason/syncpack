import 'expect-more-jest';
import { normalize } from 'path';
import { createWrapper, toJson } from '../../test/mock';
import { mockDisk } from '../../test/mock-disk';
import { DEFAULT_CONFIG } from '../constants';
import type { ProgramInput } from '../lib/get-input';
import { format } from './format';

describe('format', () => {
  it('sorts array properties alphabetically by value', () => {
    const disk = mockDisk();
    const before = { keywords: ['B', 'A'] };
    const after = { keywords: ['A', 'B'] };
    const input = {
      ...DEFAULT_CONFIG,
      sortAz: ['keywords'],
      wrappers: [createWrapper(before)],
    } as ProgramInput;
    const json = toJson(after);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    format(input, disk);
    expect(disk.writeFileSync).toHaveBeenCalledWith(
      expect.stringContaining(normalize('/some/package.json')),
      json,
    );
    expect(before).toEqual(after);
    expect(log).toHaveBeenCalledWith(
      expect.stringMatching(/✓/),
      expect.stringContaining(normalize('some/package.json')),
    );
  });
  it('sorts object properties alphabetically by key', () => {
    const disk = mockDisk();
    const before = { scripts: { B: '', A: '' } };
    const after = { scripts: { A: '', B: '' } };
    const input = {
      ...DEFAULT_CONFIG,
      sortAz: ['scripts'],
      wrappers: [createWrapper(before)],
    } as ProgramInput;
    const json = toJson(after);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    format(input, disk);
    expect(disk.writeFileSync).toHaveBeenCalledWith(
      expect.stringContaining(normalize('/some/package.json')),
      json,
    );
    expect(before).toEqual(after);
    expect(log).toHaveBeenCalledWith(
      expect.stringMatching(/✓/),
      expect.stringContaining(normalize('some/package.json')),
    );
  });
  it('sorts named properties first, then the rest alphabetically', () => {
    const disk = mockDisk();
    const before = { A: '', C: '', F: '', B: '', D: '', E: '' };
    const after = { D: '', E: '', F: '', A: '', B: '', C: '' };
    const input = {
      ...DEFAULT_CONFIG,
      sortFirst: ['D', 'E', 'F'],
      wrappers: [createWrapper(before)],
    } as ProgramInput;
    const json = toJson(after);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    format(input, disk);
    expect(disk.writeFileSync).toHaveBeenCalledWith(
      expect.stringContaining(normalize('/some/package.json')),
      json,
    );
    expect(before).toEqual(after);
    expect(log).toHaveBeenCalledWith(
      expect.stringMatching(/✓/),
      expect.stringContaining(normalize('some/package.json')),
    );
  });
  it('uses shorthand format for "bugs"', () => {
    const disk = mockDisk();
    const before = { bugs: { url: 'https://github.com/User/repo/issues' } };
    const after = { bugs: 'https://github.com/User/repo/issues' };
    const input = {
      ...DEFAULT_CONFIG,
      wrappers: [createWrapper(before)],
    } as ProgramInput;
    const json = toJson(after);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    format(input, disk);
    expect(disk.writeFileSync).toHaveBeenCalledWith(
      expect.stringContaining(normalize('/some/package.json')),
      json,
    );
    expect(before).toEqual(after);
    expect(log).toHaveBeenCalledWith(
      expect.stringMatching(/✓/),
      expect.stringContaining(normalize('some/package.json')),
    );
  });
  it('uses shorthand format for "repository"', () => {
    const disk = mockDisk();
    const before = {
      repository: { url: 'git://gitlab.com/User/repo', type: 'git' },
    };
    const after = { repository: 'git://gitlab.com/User/repo' };
    const input = {
      ...DEFAULT_CONFIG,
      wrappers: [createWrapper(before)],
    } as ProgramInput;
    const json = toJson(after);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    format(input, disk);
    expect(disk.writeFileSync).toHaveBeenCalledWith(
      expect.stringContaining(normalize('/some/package.json')),
      json,
    );
    expect(before).toEqual(after);
    expect(log).toHaveBeenCalledWith(
      expect.stringMatching(/✓/),
      expect.stringContaining(normalize('some/package.json')),
    );
  });
  it('uses github shorthand format for "repository"', () => {
    const disk = mockDisk();
    const before = {
      repository: { url: 'git://github.com/User/repo', type: 'git' },
    };
    const after = { repository: 'User/repo' };
    const input = {
      ...DEFAULT_CONFIG,
      wrappers: [createWrapper(before)],
    } as ProgramInput;
    const json = toJson(after);
    const log = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    format(input, disk);
    expect(disk.writeFileSync).toHaveBeenCalledWith(
      expect.stringContaining(normalize('/some/package.json')),
      json,
    );
    expect(before).toEqual(after);
    expect(log).toHaveBeenCalledWith(
      expect.stringMatching(/✓/),
      expect.stringContaining(normalize('some/package.json')),
    );
  });
});
