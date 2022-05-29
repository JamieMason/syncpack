module.exports = {
  collectCoverage: true,
  collectCoverageFrom: ['src/**/*.ts', '!src/**/bin*/index.ts'],
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
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
};
