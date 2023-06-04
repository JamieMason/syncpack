import 'expect-more-jest';

beforeEach(() => {
  jest.resetModules();
  jest.mock('cosmiconfig', () => {
    const load = jest.fn();
    const search = jest.fn();
    return {
      cosmiconfigSync() {
        return {
          load,
          search,
        };
      },
    };
  });
});

const configPath = '/path/to/.syncpackrc';

describe('readConfigFileSync', () => {
  it('searches parent directories when no file path is provided', () => {
    const client = require('cosmiconfig').cosmiconfigSync();
    const { effects } = require('./effects');
    expect(effects.readConfigFileSync()).toBeEmptyObject();
    expect(client.load).not.toHaveBeenCalled();
    expect(client.search).toHaveBeenCalledTimes(1);
  });

  describe('when a path to a config file is provided', () => {
    describe('when the file can be found', () => {
      it('return its config', () => {
        const client = require('cosmiconfig').cosmiconfigSync();
        const { effects } = require('./effects');
        const mockConfig = { sortAz: ['foo'] };
        client.load.mockReturnValue({ config: mockConfig });
        expect(effects.readConfigFileSync(configPath)).toEqual(mockConfig);
        expect(client.load).toHaveBeenCalledTimes(1);
        expect(client.search).not.toHaveBeenCalled();
      });
    });

    describe('when the file can not be found', () => {
      it('returns an empty object', () => {
        const client = require('cosmiconfig').cosmiconfigSync();
        const { effects } = require('./effects');
        client.load.mockImplementation(() => {
          throw new Error('NOPE');
        });
        expect(effects.readConfigFileSync(configPath)).toBeEmptyObject();
        expect(client.load).toHaveBeenCalledTimes(1);
        expect(client.search).not.toHaveBeenCalled();
      });
    });
  });
});
