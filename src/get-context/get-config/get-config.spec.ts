import { R } from '@mobily/ts-belt';
import 'expect-more-jest';
import { getConfig } from '.';
import { mockDisk } from '../../../test/mock-disk';
import type { BaseError } from '../../lib/error';
import type { Syncpack } from '../../types';

describe('enabledTypes', () => {
  const all = 'dev,overrides,peer,pnpmOverrides,prod,resolutions,workspace';

  function getNames(config: R.Result<Syncpack.Config.Private, BaseError>) {
    return R.getExn(config).enabledTypes.map(({ name }) => name);
  }

  it('enables all when nothing is set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    expect(getNames(config)).toEqual(all.split(','));
  });

  it('enables named CLI types when set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, { types: 'dev,prod' });
    expect(getNames(config)).toEqual(['dev', 'prod']);
  });

  it('enables named CLI types when set and ignores config when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ dependencyTypes: ['peer'] });
    const config = getConfig(disk, { types: 'dev,prod' });
    expect(getNames(config)).toEqual(['dev', 'prod']);
  });

  it('enables config types when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({
      dependencyTypes: ['dev', 'peer'],
    });
    const config = getConfig(disk, {});
    expect(getNames(config)).toEqual(['dev', 'peer']);
  });

  it('enables custom config types when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({
      dependencyTypes: ['someName'],
      customTypes: {
        someName: { path: 'foo', strategy: 'name@version' },
      },
    });
    const configOnly = getConfig(disk, {});
    expect(getNames(configOnly)).toEqual(['someName']);
    const configAndCliTypes = getConfig(disk, { types: 'prod,someName' });
    expect(getNames(configAndCliTypes)).toEqual(['prod', 'someName']);
  });
});

describe('filter', () => {
  it('uses default when not set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    expect(R.getExn(config).filter).toEqual('.');
  });

  it('uses CLI value when set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, { filter: 'foo' });
    expect(R.getExn(config).filter).toEqual('foo');
  });

  it('uses config value when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ filter: 'bar' });
    const config = getConfig(disk, {});
    expect(R.getExn(config).filter).toEqual('bar');
  });

  it('uses CLI value when config and CLI are set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ filter: 'bar' });
    const config = getConfig(disk, { filter: 'foo' });
    expect(R.getExn(config).filter).toEqual('foo');
  });
});

describe('indent', () => {
  it('uses default when not set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    expect(R.getExn(config).indent).toEqual('  ');
  });

  it('uses CLI value when set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, { indent: '\t' });
    expect(R.getExn(config).indent).toEqual('\t');
  });

  it('uses config value when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ indent: '\t' });
    const config = getConfig(disk, {});
    expect(R.getExn(config).indent).toEqual('\t');
  });

  it('uses CLI value when config and CLI are set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ indent: '\t' });
    const config = getConfig(disk, { indent: '    ' });
    expect(R.getExn(config).indent).toEqual('    ');
  });
});

describe('semverRange', () => {
  it('uses default when not set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    expect(R.getExn(config).semverRange).toEqual('');
  });

  it('uses CLI value when set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, { semverRange: '^' });
    expect(R.getExn(config).semverRange).toEqual('^');
  });

  it('uses config value when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ semverRange: '~' });
    const config = getConfig(disk, {});
    expect(R.getExn(config).semverRange).toEqual('~');
  });

  it('uses CLI value when config and CLI are set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ semverRange: '^' });
    const config = getConfig(disk, { semverRange: '*' });
    expect(R.getExn(config).semverRange).toEqual('*');
  });
});

describe('source', () => {
  it('uses default when not set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, {});
    expect(R.getExn(config).source).toEqual([]);
  });

  it('uses CLI value when set', () => {
    const disk = mockDisk();
    const config = getConfig(disk, { source: ['apps/*'] });
    expect(R.getExn(config).source).toEqual(['apps/*']);
  });

  it('uses config value when set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ source: ['projects/*'] });
    const config = getConfig(disk, {});
    expect(R.getExn(config).source).toEqual(['projects/*']);
  });

  it('uses CLI value when config and CLI are set', () => {
    const disk = mockDisk();
    disk.readConfigFileSync.mockReturnValue({ source: ['projects/*'] });
    const config = getConfig(disk, { source: ['apps/*'] });
    expect(R.getExn(config).source).toEqual(['apps/*']);
  });
});
