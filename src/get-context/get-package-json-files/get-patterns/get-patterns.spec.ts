import { R } from '@mobily/ts-belt';
import { getPatterns } from '.';
import { mockDisk } from '../../../../test/mock-disk';
import { DEFAULT_SOURCES } from '../../../constants';
import { getConfig } from '../../get-config';

it('returns R.Ok of default patterns when nothing is available', () => {
  const disk = mockDisk();
  const config = getConfig(disk, {});
  expect(getPatterns(disk)(R.getExn(config))).toEqual(R.Ok(DEFAULT_SOURCES));
});

it('CLI --source options take precedence', () => {
  const disk = mockDisk();
  const config = getConfig(disk, { source: ['foo/package.json'] });
  expect(getPatterns(disk)(R.getExn(config))).toEqual(
    R.Ok(['package.json', 'foo/package.json']),
  );
});

describe('Yarn takes precedence after CLI --source options', () => {
  it('returns R.Ok of strings when valid', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    disk.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return JSON.stringify({ workspaces: ['yarn/*'] });
      }
    });
    expect(getPatterns(disk)(R.getExn(config))).toEqual(
      R.Ok(['package.json', 'yarn/*/package.json']),
    );
  });

  it('returns R.Ok of default patterns when Yarn config is invalid', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    disk.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return 'wut?';
      }
    });
    expect(getPatterns(disk)(R.getExn(config))).toEqual(R.Ok(DEFAULT_SOURCES));
  });
});

describe('Pnpm takes precedence after Yarn', () => {
  it('returns R.Ok of strings when valid', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    disk.readYamlFileSync.mockImplementation(() => ({
      packages: ['pnpm/*'],
    }));
    expect(getPatterns(disk)(R.getExn(config))).toEqual(
      R.Ok(['package.json', 'pnpm/*/package.json']),
    );
  });

  it('returns R.Ok of default patterns when Pnpm config is invalid', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    disk.readYamlFileSync.mockImplementation(() => {
      throw new Error('Reason does not matter to this test');
    });
    expect(getPatterns(disk)(R.getExn(config))).toEqual(R.Ok(DEFAULT_SOURCES));
  });
});

describe('Lerna takes precedence after Pnpm', () => {
  it('returns R.Ok of strings when valid', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    disk.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('lerna.json')) {
        return JSON.stringify({ packages: ['lerna/*'] });
      }
    });
    expect(getPatterns(disk)(R.getExn(config))).toEqual(
      R.Ok(['package.json', 'lerna/*/package.json']),
    );
  });

  it('returns R.Ok of default patterns when Yarn config is invalid', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    disk.readFileSync.mockImplementation((filePath) => {
      if (filePath.endsWith('package.json')) {
        return 'wut?';
      }
    });
    expect(getPatterns(disk)(R.getExn(config))).toEqual(R.Ok(DEFAULT_SOURCES));
  });
});
