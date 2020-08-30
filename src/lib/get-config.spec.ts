import { DEFAULT_CONFIG, SyncpackConfig } from '../constants';

describe('getConfig', () => {
  let getConfig = require('./get-config').getConfig;

  const setConfigFileTo = (value: Partial<SyncpackConfig>) => {
    jest.resetModules();
    jest.mock('cosmiconfig', () => ({ cosmiconfigSync: jest.fn() }));
    const { cosmiconfigSync } = require('cosmiconfig');
    cosmiconfigSync.mockReturnValue({ search: () => ({ config: value }) });
    getConfig = require('./get-config').getConfig;
  };

  const removeConfigFile = () => {
    jest.resetModules();
    jest.mock('cosmiconfig', () => ({ cosmiconfigSync: jest.fn() }));
    const { cosmiconfigSync } = require('cosmiconfig');
    cosmiconfigSync.mockReturnValue({ search: () => null });
    getConfig = require('./get-config').getConfig;
  };

  it('returns default when config and CLI option are not used', () => {
    removeConfigFile();
    expect(getConfig({})).toHaveProperty('source', DEFAULT_CONFIG.source);
  });

  it('returns config when CLI option is not used', () => {
    setConfigFileTo({ source: ['./from-config'] });
    expect(getConfig({})).toHaveProperty('source', ['./from-config']);
  });

  it('returns CLI option when used', () => {
    setConfigFileTo({ source: ['./from-config'] });
    expect(getConfig({ source: ['./from-option'] })).toHaveProperty('source', ['./from-option']);
  });

  it('merges defaults, config, and CLI options', () => {
    setConfigFileTo({ source: ['./from-config'] });
    expect(getConfig({ filter: 'syncpack', semverRange: '~', sortAz: ['overridden'] })).toEqual({
      dev: true,
      filter: 'syncpack',
      indent: '  ',
      peer: true,
      prod: true,
      semverRange: '~',
      sortAz: ['overridden'],
      sortFirst: ['name', 'description', 'version', 'author'],
      source: ['./from-config'],
      versionGroups: [],
    });
  });

  it('merges config-only options', () => {
    setConfigFileTo({
      sortAz: ['overridden'],
      sortFirst: ['overridden'],
      versionGroups: [{ dependencies: ['chalk'], packages: ['foo', 'bar'] }],
    });
    expect(getConfig({ filter: 'syncpack', semverRange: '~', sortAz: ['overridden'] })).toEqual(
      expect.objectContaining({
        versionGroups: [{ dependencies: ['chalk'], packages: ['foo', 'bar'] }],
      }),
    );
  });

  it('overrides all dependency types when any CLI option is used', () => {
    setConfigFileTo({ dev: true, peer: true, prod: true });
    expect(getConfig({ prod: true })).toEqual(
      expect.objectContaining({
        dev: false,
        peer: false,
        prod: true,
      }),
    );
  });
});
