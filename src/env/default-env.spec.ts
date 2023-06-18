import 'expect-more-jest';

beforeEach(() => {
  jest.resetModules();
  jest.mock('cosmiconfig', () => {
    const load = jest.fn();
    const search = jest.fn();
    return {
      cosmiconfigSync() {
        return { load, search };
      },
    };
  });
});

const configPath = '/path/to/.syncpackrc';

describe('readConfigFileSync', () => {
  it('searches parent directories when no file path is provided', async () => {
    const cosmiconfig = await import('cosmiconfig');
    const { defaultEnv } = await import('./default-env');
    const client = cosmiconfig.cosmiconfigSync('syncpack');
    expect(defaultEnv.readConfigFileSync()).toBeEmptyObject();
    expect(client.load).not.toHaveBeenCalled();
    expect(client.search).toHaveBeenCalledTimes(1);
  });

  describe('when a path to a config file is provided', () => {
    describe('when the file can be found', () => {
      it('return its config', async () => {
        const cosmiconfig = await import('cosmiconfig');
        const { defaultEnv } = await import('./default-env');
        const client = cosmiconfig.cosmiconfigSync('syncpack');
        const mockConfig = { sortAz: ['foo'] };
        (client.load as jest.Mock).mockReturnValue({ config: mockConfig });
        expect(defaultEnv.readConfigFileSync(configPath)).toEqual(mockConfig);
        expect(client.load).toHaveBeenCalledTimes(1);
        expect(client.search).not.toHaveBeenCalled();
      });
    });

    describe('when the file can not be found', () => {
      it('throws an error', async () => {
        const cosmiconfig = await import('cosmiconfig');
        const { defaultEnv } = await import('./default-env');
        const client = cosmiconfig.cosmiconfigSync('syncpack');
        (client.load as jest.Mock).mockImplementation(() => {
          throw new Error('NOPE');
        });
        expect(() => defaultEnv.readConfigFileSync(configPath)).toThrow();
        expect(client.load).toHaveBeenCalledTimes(1);
        expect(client.search).not.toHaveBeenCalled();
      });
    });
  });
});
