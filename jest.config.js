module.exports = {
  collectCoverage: true,
  collectCoverageFrom: [
    'src/**/*.ts',
    '!src/bin.ts',
    '!src/bin*/index.ts',
    '!src/lib/disk.ts',
    '!src/lib/log.ts',
    '!src/option.ts',
  ],
  coverageReporters: ['html', 'lcov'],
  coverageThreshold: {
    global: {
      branches: 79,
      functions: 93,
      lines: 93,
      statements: 92,
    },
  },
  moduleFileExtensions: ['ts', 'js'],
  setupFiles: ['<rootDir>/test/jest.setup.ts'],
  testMatch: ['<rootDir>/src/**/*.spec.(ts|js)'],
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
};
