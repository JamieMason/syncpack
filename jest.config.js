module.exports = {
  collectCoverage: true,
  collectCoverageFrom: ['src/**/*.ts', '!src/bin.ts', '!src/bin*/index.ts'],
  coverageReporters: ['html', 'lcov'],
  coverageThreshold: {
    global: {
      statements: 82,
      branches: 75,
      lines: 82,
      functions: 75,
    },
  },
  moduleFileExtensions: ['ts', 'js'],
  setupFiles: ['<rootDir>/test/jest.setup.ts'],
  setupFilesAfterEnv: ['<rootDir>/test/jest.setup-after-env.ts'],
  testMatch: ['<rootDir>/src/**/*.spec.ts'],
  transform: {
    '^.+\\.ts$': ['ts-jest', { isolatedModules: true }],
  },
};
