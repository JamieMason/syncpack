export interface MockEffects {
  readonly askForChoice: jest.Mock<any, any>;
  readonly askForInput: jest.Mock<any, any>;
  readonly globSync: jest.Mock<any, any>;
  readonly process: {
    exit: jest.Mock<any, any>;
  };
  readonly readConfigFileSync: jest.Mock<any, any>;
  readonly readFileSync: jest.Mock<any, any>;
  readonly readYamlFileSync: jest.Mock<any, any>;
  readonly removeSync: jest.Mock<any, any>;
  readonly writeFileSync: jest.Mock<any, any>;
}

export function mockEffects(): MockEffects {
  return {
    askForChoice: jest.fn(() => Promise.resolve()),
    askForInput: jest.fn(() => Promise.resolve()),
    globSync: jest.fn(() => []),
    process: {
      exit: jest.fn(() => []),
    },
    readConfigFileSync: jest.fn(() => ({})),
    readFileSync: jest.fn(() => ''),
    readYamlFileSync: jest.fn(() => ({})),
    removeSync: jest.fn(),
    writeFileSync: jest.fn(),
  };
}
